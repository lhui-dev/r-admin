use sqlx::{FromRow, Postgres, Transaction};
use std::collections::{HashMap, HashSet};

use crate::{
    common::error::{AppError, AppResult},
    modules::system_dept::dto::{
        CreateDeptRequest, DeptDetailData, DeptMutationData, DeptStatusMutationData, DeptTreeData,
        DeptTreeItem, DeptTreeQuery, PatchField, UpdateDeptRequest, UpdateDeptStatusRequest,
    },
    state::AppState,
};

#[derive(Debug, Clone, FromRow)]
struct DeptRow {
    id: i64,
    parent_id: i64,
    dept_name: String,
    dept_code: Option<String>,
    leader_user_id: Option<i64>,
    leader_name: Option<String>,
    sort_no: i32,
    status: i16,
    remark: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct DeptEditRow {
    id: i64,
    parent_id: i64,
    dept_name: String,
    dept_code: Option<String>,
    leader_user_id: Option<i64>,
    sort_no: i32,
    status: i16,
    remark: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
struct DeptIdentityRow {
    id: i64,
    dept_name: String,
}

#[derive(Debug, Clone, FromRow)]
struct ExistsRow {
    exists_flag: bool,
}

#[derive(Debug, Clone, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, Clone, FromRow)]
struct NextIdRow {
    next_id: i64,
}

#[derive(Debug)]
struct DeptInput {
    parent_id: i64,
    dept_name: String,
    dept_code: Option<String>,
    leader_user_id: Option<i64>,
    sort_no: i32,
    status: i16,
    remark: Option<String>,
}

pub async fn list_dept_tree(state: &AppState, query: DeptTreeQuery) -> AppResult<DeptTreeData> {
    let keyword = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{value}%"));
    let status = query.status.map(validate_status).transpose()?;

    let rows = sqlx::query_as::<_, DeptRow>(
        r#"
        SELECT
            d.id,
            d.parent_id,
            d.dept_name,
            d.dept_code,
            d.leader_user_id,
            u.nickname AS leader_name,
            d.sort_no,
            d.status,
            d.remark,
            TO_CHAR(d.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(d.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_dept d
        LEFT JOIN sys_user u ON u.id = d.leader_user_id
        WHERE d.is_deleted = FALSE
          AND d.id <> 0
          AND ($1::TEXT IS NULL
                OR d.dept_name ILIKE $1
                OR COALESCE(d.dept_code, '') ILIKE $1
                OR COALESCE(u.nickname, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR d.status = $2)
        ORDER BY d.sort_no, d.id
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .fetch_all(&state.db)
    .await?;

    Ok(DeptTreeData {
        items: build_dept_tree(rows),
    })
}

pub async fn get_dept_detail(state: &AppState, dept_id: i64) -> AppResult<DeptDetailData> {
    let row = find_dept_row(state, dept_id)
        .await?
        .ok_or_else(|| AppError::not_found("department not found"))?;

    Ok(map_dept_item(row, Vec::new()))
}

pub async fn create_dept(
    state: &AppState,
    operator_user_id: i64,
    payload: CreateDeptRequest,
) -> AppResult<DeptMutationData> {
    let input = normalize_create_input(payload)?;
    let mut tx = state.db.begin().await?;

    ensure_parent_exists(&mut tx, input.parent_id).await?;
    ensure_leader_exists(&mut tx, input.leader_user_id).await?;
    ensure_unique_dept_code(&mut tx, input.dept_code.as_deref(), None).await?;

    let new_dept_id = next_dept_id(&mut tx).await?;

    sqlx::query(
        r#"
        INSERT INTO sys_dept (
            id,
            parent_id,
            dept_name,
            dept_code,
            leader_user_id,
            sort_no,
            status,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $8, FALSE, $9
        )
        "#,
    )
    .bind(new_dept_id)
    .bind(input.parent_id)
    .bind(&input.dept_name)
    .bind(&input.dept_code)
    .bind(input.leader_user_id)
    .bind(input.sort_no)
    .bind(input.status)
    .bind(operator_user_id)
    .bind(&input.remark)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(DeptMutationData {
        id: new_dept_id,
        dept_name: input.dept_name,
    })
}

pub async fn update_dept(
    state: &AppState,
    operator_user_id: i64,
    dept_id: i64,
    payload: UpdateDeptRequest,
) -> AppResult<DeptMutationData> {
    ensure_not_root_dept(dept_id)?;
    if !payload.has_any_field() {
        return Err(AppError::bad_request(
            "at least one updatable field is required",
        ));
    }

    let mut tx = state.db.begin().await?;
    let current_dept = find_dept_edit_row(&mut tx, dept_id)
        .await?
        .ok_or_else(|| AppError::not_found("department not found"))?;
    let input = normalize_update_input(&current_dept, payload)?;

    ensure_parent_exists(&mut tx, input.parent_id).await?;
    ensure_parent_not_descendant(&mut tx, dept_id, input.parent_id).await?;
    ensure_leader_exists(&mut tx, input.leader_user_id).await?;
    ensure_unique_dept_code(&mut tx, input.dept_code.as_deref(), Some(dept_id)).await?;

    sqlx::query(
        r#"
        UPDATE sys_dept
        SET parent_id = $2,
            dept_name = $3,
            dept_code = $4,
            leader_user_id = $5,
            sort_no = $6,
            status = $7,
            remark = $8,
            updated_at = NOW(),
            updated_by = $9
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(dept_id)
    .bind(input.parent_id)
    .bind(&input.dept_name)
    .bind(&input.dept_code)
    .bind(input.leader_user_id)
    .bind(input.sort_no)
    .bind(input.status)
    .bind(&input.remark)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(DeptMutationData {
        id: current_dept.id,
        dept_name: input.dept_name,
    })
}

pub async fn update_dept_status(
    state: &AppState,
    operator_user_id: i64,
    dept_id: i64,
    payload: UpdateDeptStatusRequest,
) -> AppResult<DeptStatusMutationData> {
    ensure_not_root_dept(dept_id)?;

    let status = validate_status(payload.status)?;
    let mut tx = state.db.begin().await?;
    let current_dept = find_dept_identity(&mut tx, dept_id)
        .await?
        .ok_or_else(|| AppError::not_found("department not found"))?;

    sqlx::query(
        r#"
        UPDATE sys_dept
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(dept_id)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(DeptStatusMutationData {
        id: current_dept.id,
        dept_name: current_dept.dept_name,
        status,
    })
}

pub async fn delete_dept(
    state: &AppState,
    operator_user_id: i64,
    dept_id: i64,
) -> AppResult<DeptMutationData> {
    ensure_not_root_dept(dept_id)?;

    let mut tx = state.db.begin().await?;
    let current_dept = find_dept_identity(&mut tx, dept_id)
        .await?
        .ok_or_else(|| AppError::not_found("department not found"))?;

    ensure_dept_has_no_children(&mut tx, dept_id).await?;
    ensure_dept_has_no_users(&mut tx, dept_id).await?;

    sqlx::query(
        r#"
        UPDATE sys_dept
        SET is_deleted = TRUE,
            updated_at = NOW(),
            updated_by = $2
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(dept_id)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(DeptMutationData {
        id: current_dept.id,
        dept_name: current_dept.dept_name,
    })
}

fn build_dept_tree(rows: Vec<DeptRow>) -> Vec<DeptTreeItem> {
    let existing_ids = rows.iter().map(|row| row.id).collect::<HashSet<_>>();
    let mut children_by_parent: HashMap<i64, Vec<DeptRow>> = HashMap::new();
    let mut root_parent_ids = Vec::new();

    for row in rows {
        if row.parent_id == 0 || !existing_ids.contains(&row.parent_id) {
            root_parent_ids.push(row.parent_id);
        }
        children_by_parent
            .entry(row.parent_id)
            .or_default()
            .push(row);
    }

    root_parent_ids.sort_unstable();
    root_parent_ids.dedup();

    root_parent_ids
        .into_iter()
        .flat_map(|parent_id| build_dept_nodes(parent_id, &mut children_by_parent))
        .collect()
}

fn build_dept_nodes(
    parent_id: i64,
    children_by_parent: &mut HashMap<i64, Vec<DeptRow>>,
) -> Vec<DeptTreeItem> {
    let Some(mut rows) = children_by_parent.remove(&parent_id) else {
        return Vec::new();
    };

    rows.sort_by_key(|row| (row.sort_no, row.id));
    rows.into_iter()
        .map(|row| {
            let children = build_dept_nodes(row.id, children_by_parent);
            map_dept_item(row, children)
        })
        .collect()
}

fn map_dept_item(row: DeptRow, children: Vec<DeptTreeItem>) -> DeptTreeItem {
    DeptTreeItem {
        id: row.id,
        parent_id: row.parent_id,
        dept_name: row.dept_name,
        dept_code: row.dept_code,
        leader_user_id: row.leader_user_id,
        leader_name: row.leader_name,
        sort_no: row.sort_no,
        status: row.status,
        remark: row.remark,
        created_at: row.created_at,
        updated_at: row.updated_at,
        children,
    }
}

async fn find_dept_row(state: &AppState, dept_id: i64) -> AppResult<Option<DeptRow>> {
    sqlx::query_as::<_, DeptRow>(
        r#"
        SELECT
            d.id,
            d.parent_id,
            d.dept_name,
            d.dept_code,
            d.leader_user_id,
            u.nickname AS leader_name,
            d.sort_no,
            d.status,
            d.remark,
            TO_CHAR(d.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(d.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_dept d
        LEFT JOIN sys_user u ON u.id = d.leader_user_id
        WHERE d.id = $1
          AND d.id <> 0
          AND d.is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(dept_id)
    .fetch_optional(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_dept_edit_row(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: i64,
) -> AppResult<Option<DeptEditRow>> {
    sqlx::query_as::<_, DeptEditRow>(
        r#"
        SELECT
            id,
            parent_id,
            dept_name,
            dept_code,
            leader_user_id,
            sort_no,
            status,
            remark
        FROM sys_dept
        WHERE id = $1
          AND id <> 0
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(dept_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn find_dept_identity(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: i64,
) -> AppResult<Option<DeptIdentityRow>> {
    sqlx::query_as::<_, DeptIdentityRow>(
        r#"
        SELECT id, dept_name
        FROM sys_dept
        WHERE id = $1
          AND id <> 0
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(dept_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn next_dept_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 100) + 10 AS next_id
        FROM sys_dept
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn ensure_parent_exists(tx: &mut Transaction<'_, Postgres>, parent_id: i64) -> AppResult<()> {
    if parent_id < 0 {
        return Err(AppError::bad_request("parent_id cannot be negative"));
    }

    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_dept
            WHERE id = $1
              AND is_deleted = FALSE
        ) AS exists_flag
        "#,
    )
    .bind(parent_id)
    .fetch_one(&mut **tx)
    .await?;

    if !row.exists_flag {
        return Err(AppError::bad_request("parent department does not exist"));
    }

    Ok(())
}

async fn ensure_parent_not_descendant(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: i64,
    parent_id: i64,
) -> AppResult<()> {
    if parent_id == dept_id {
        return Err(AppError::bad_request("parent department cannot be self"));
    }

    let mut current_parent_id = parent_id;
    let mut visited = HashSet::new();
    while current_parent_id != 0 {
        if !visited.insert(current_parent_id) {
            return Err(AppError::bad_request("department parent chain has a cycle"));
        }

        if current_parent_id == dept_id {
            return Err(AppError::bad_request(
                "parent department cannot be a descendant of current department",
            ));
        }

        let next_parent_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT parent_id
            FROM sys_dept
            WHERE id = $1
              AND is_deleted = FALSE
            LIMIT 1
            "#,
        )
        .bind(current_parent_id)
        .fetch_optional(&mut **tx)
        .await?;

        let Some(next_parent_id) = next_parent_id else {
            break;
        };
        current_parent_id = next_parent_id;
    }

    Ok(())
}

async fn ensure_leader_exists(
    tx: &mut Transaction<'_, Postgres>,
    leader_user_id: Option<i64>,
) -> AppResult<()> {
    let Some(leader_user_id) = leader_user_id else {
        return Ok(());
    };

    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_user
            WHERE id = $1
              AND is_deleted = FALSE
        ) AS exists_flag
        "#,
    )
    .bind(leader_user_id)
    .fetch_one(&mut **tx)
    .await?;

    if !row.exists_flag {
        return Err(AppError::bad_request("leader user does not exist"));
    }

    Ok(())
}

async fn ensure_unique_dept_code(
    tx: &mut Transaction<'_, Postgres>,
    dept_code: Option<&str>,
    exclude_dept_id: Option<i64>,
) -> AppResult<()> {
    let Some(dept_code) = dept_code else {
        return Ok(());
    };

    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_dept
            WHERE dept_code = $1
              AND is_deleted = FALSE
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(dept_code)
    .bind(exclude_dept_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.exists_flag {
        return Err(AppError::conflict("dept_code already exists"));
    }

    Ok(())
}

async fn ensure_dept_has_no_children(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: i64,
) -> AppResult<()> {
    let row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_dept
        WHERE parent_id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(dept_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.total > 0 {
        return Err(AppError::conflict(
            "department has child nodes and cannot be deleted",
        ));
    }

    Ok(())
}

async fn ensure_dept_has_no_users(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: i64,
) -> AppResult<()> {
    let row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_user
        WHERE dept_id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(dept_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.total > 0 {
        return Err(AppError::conflict(
            "department has assigned users and cannot be deleted",
        ));
    }

    Ok(())
}

fn normalize_create_input(payload: CreateDeptRequest) -> AppResult<DeptInput> {
    Ok(DeptInput {
        parent_id: validate_parent_id(payload.parent_id.unwrap_or(0))?,
        dept_name: require_non_empty(payload.dept_name, "dept_name")?,
        dept_code: normalize_optional_field(payload.dept_code),
        leader_user_id: payload.leader_user_id,
        sort_no: payload.sort_no.unwrap_or(0),
        status: validate_status(payload.status.unwrap_or(1))?,
        remark: normalize_optional_field(payload.remark),
    })
}

fn normalize_update_input(
    current_dept: &DeptEditRow,
    payload: UpdateDeptRequest,
) -> AppResult<DeptInput> {
    Ok(DeptInput {
        parent_id: payload
            .parent_id
            .map(validate_parent_id)
            .transpose()?
            .unwrap_or(current_dept.parent_id),
        dept_name: payload
            .dept_name
            .map(|value| require_non_empty(value, "dept_name"))
            .transpose()?
            .unwrap_or_else(|| current_dept.dept_name.clone()),
        dept_code: payload
            .dept_code
            .map_nullable_string_update(|| current_dept.dept_code.clone()),
        leader_user_id: match payload.leader_user_id {
            PatchField::Missing => current_dept.leader_user_id,
            PatchField::Null => None,
            PatchField::Value(value) => Some(value),
        },
        sort_no: payload.sort_no.unwrap_or(current_dept.sort_no),
        status: validate_status(payload.status.unwrap_or(current_dept.status))?,
        remark: payload
            .remark
            .map_nullable_string_update(|| current_dept.remark.clone()),
    })
}

trait NullableStringUpdate {
    fn map_nullable_string_update<F>(self, fallback: F) -> Option<String>
    where
        F: FnOnce() -> Option<String>;
}

impl NullableStringUpdate for PatchField<String> {
    fn map_nullable_string_update<F>(self, fallback: F) -> Option<String>
    where
        F: FnOnce() -> Option<String>,
    {
        match self {
            PatchField::Missing => fallback(),
            PatchField::Null => None,
            PatchField::Value(value) => normalize_optional_field(Some(value)),
        }
    }
}

fn validate_parent_id(parent_id: i64) -> AppResult<i64> {
    if parent_id < 0 {
        return Err(AppError::bad_request("parent_id cannot be negative"));
    }

    Ok(parent_id)
}

fn validate_status(status: i16) -> AppResult<i16> {
    if status != 0 && status != 1 {
        return Err(AppError::bad_request("status must be 0 or 1"));
    }

    Ok(status)
}

fn ensure_not_root_dept(dept_id: i64) -> AppResult<()> {
    if dept_id == 0 {
        return Err(AppError::bad_request("root department cannot be modified"));
    }

    Ok(())
}

fn require_non_empty(value: String, field_name: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request(format!("{field_name} is required")));
    }

    Ok(trimmed.to_string())
}

fn normalize_optional_field(value: Option<String>) -> Option<String> {
    value.and_then(|item| {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}
