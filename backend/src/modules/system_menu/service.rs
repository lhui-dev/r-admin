use sqlx::{FromRow, Postgres, Transaction};
use std::collections::{HashMap, HashSet};

use crate::{
    common::error::{AppError, AppResult},
    modules::system_menu::dto::{
        CreateMenuRequest, MenuDetailData, MenuMutationData, MenuStatusMutationData, MenuTreeData,
        MenuTreeItem, MenuTreeQuery, UpdateMenuRequest, UpdateMenuStatusRequest,
    },
    state::AppState,
};

#[derive(Debug, Clone, FromRow)]
struct MenuRow {
    id: i64,
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    route_name: Option<String>,
    route_path: Option<String>,
    component_path: Option<String>,
    permission_code: Option<String>,
    icon: Option<String>,
    sort_no: i32,
    visible: bool,
    keep_alive: bool,
    is_external: bool,
    status: i16,
    remark: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct MenuIdentityRow {
    id: i64,
    menu_name: String,
    menu_type: String,
    permission_code: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
struct MenuEditRow {
    id: i64,
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    route_name: Option<String>,
    route_path: Option<String>,
    component_path: Option<String>,
    permission_code: Option<String>,
    icon: Option<String>,
    sort_no: i32,
    visible: bool,
    keep_alive: bool,
    is_external: bool,
    status: i16,
    remark: Option<String>,
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
struct MenuInput {
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    route_name: Option<String>,
    route_path: Option<String>,
    component_path: Option<String>,
    permission_code: Option<String>,
    icon: Option<String>,
    sort_no: i32,
    visible: bool,
    keep_alive: bool,
    is_external: bool,
    status: i16,
    remark: Option<String>,
}

pub async fn list_menu_tree(state: &AppState, query: MenuTreeQuery) -> AppResult<MenuTreeData> {
    let keyword = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{value}%"));
    let status = query.status.map(validate_status).transpose()?;
    let menu_type = query
        .menu_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(normalize_menu_type)
        .transpose()?;

    let rows = sqlx::query_as::<_, MenuRow>(
        r#"
        SELECT
            id,
            parent_id,
            menu_name,
            menu_type,
            route_name,
            route_path,
            component_path,
            permission_code,
            icon,
            sort_no,
            visible,
            keep_alive,
            is_external,
            status,
            remark,
            TO_CHAR(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_menu
        WHERE is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR menu_name ILIKE $1
                OR COALESCE(route_name, '') ILIKE $1
                OR COALESCE(route_path, '') ILIKE $1
                OR COALESCE(component_path, '') ILIKE $1
                OR COALESCE(permission_code, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR status = $2)
          AND ($3::TEXT IS NULL OR menu_type = $3)
        ORDER BY sort_no, id
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .bind(menu_type.as_deref())
    .fetch_all(&state.db)
    .await?;

    Ok(MenuTreeData {
        items: build_menu_tree(rows),
    })
}

pub async fn get_menu_detail(state: &AppState, menu_id: i64) -> AppResult<MenuDetailData> {
    let row = find_menu_row(state, menu_id)
        .await?
        .ok_or_else(|| AppError::not_found("menu not found"))?;

    Ok(map_menu_item(row, Vec::new()))
}

pub async fn create_menu(
    state: &AppState,
    operator_user_id: i64,
    payload: CreateMenuRequest,
) -> AppResult<MenuMutationData> {
    let input = normalize_create_input(payload)?;
    let mut tx = state.db.begin().await?;

    ensure_parent_exists(&mut tx, input.parent_id).await?;
    ensure_unique_permission_code(&mut tx, input.permission_code.as_deref(), None).await?;

    let new_menu_id = next_menu_id(&mut tx).await?;

    sqlx::query(
        r#"
        INSERT INTO sys_menu (
            id,
            parent_id,
            menu_name,
            menu_type,
            route_name,
            route_path,
            component_path,
            permission_code,
            icon,
            sort_no,
            visible,
            keep_alive,
            is_external,
            status,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, NOW(), NOW(), $15, $15, FALSE, $16
        )
        "#,
    )
    .bind(new_menu_id)
    .bind(input.parent_id)
    .bind(&input.menu_name)
    .bind(&input.menu_type)
    .bind(&input.route_name)
    .bind(&input.route_path)
    .bind(&input.component_path)
    .bind(&input.permission_code)
    .bind(&input.icon)
    .bind(input.sort_no)
    .bind(input.visible)
    .bind(input.keep_alive)
    .bind(input.is_external)
    .bind(input.status)
    .bind(operator_user_id)
    .bind(&input.remark)
    .execute(&mut *tx)
    .await?;

    sync_menu_permission(&mut tx, operator_user_id, &input).await?;

    tx.commit().await?;

    Ok(MenuMutationData {
        id: new_menu_id,
        menu_name: input.menu_name,
        menu_type: input.menu_type,
    })
}

pub async fn update_menu(
    state: &AppState,
    operator_user_id: i64,
    menu_id: i64,
    payload: UpdateMenuRequest,
) -> AppResult<MenuMutationData> {
    let mut tx = state.db.begin().await?;
    let current_menu = find_menu_edit_row(&mut tx, menu_id)
        .await?
        .ok_or_else(|| AppError::not_found("menu not found"))?;
    let input = normalize_update_input(&current_menu, payload)?;

    ensure_parent_exists(&mut tx, input.parent_id).await?;
    ensure_parent_not_descendant(&mut tx, menu_id, input.parent_id).await?;
    ensure_unique_permission_code(&mut tx, input.permission_code.as_deref(), Some(menu_id)).await?;
    if current_menu.permission_code != input.permission_code {
        ensure_menu_permission_not_assigned(&mut tx, current_menu.permission_code.as_deref())
            .await?;
    }

    sqlx::query(
        r#"
        UPDATE sys_menu
        SET parent_id = $2,
            menu_name = $3,
            menu_type = $4,
            route_name = $5,
            route_path = $6,
            component_path = $7,
            permission_code = $8,
            icon = $9,
            sort_no = $10,
            visible = $11,
            keep_alive = $12,
            is_external = $13,
            status = $14,
            remark = $15,
            updated_at = NOW(),
            updated_by = $16
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(menu_id)
    .bind(input.parent_id)
    .bind(&input.menu_name)
    .bind(&input.menu_type)
    .bind(&input.route_name)
    .bind(&input.route_path)
    .bind(&input.component_path)
    .bind(&input.permission_code)
    .bind(&input.icon)
    .bind(input.sort_no)
    .bind(input.visible)
    .bind(input.keep_alive)
    .bind(input.is_external)
    .bind(input.status)
    .bind(&input.remark)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    if current_menu.permission_code != input.permission_code {
        deactivate_menu_permission(
            &mut tx,
            operator_user_id,
            current_menu.permission_code.as_deref(),
        )
        .await?;
    }
    sync_menu_permission(&mut tx, operator_user_id, &input).await?;

    tx.commit().await?;

    Ok(MenuMutationData {
        id: current_menu.id,
        menu_name: input.menu_name,
        menu_type: input.menu_type,
    })
}

pub async fn update_menu_status(
    state: &AppState,
    operator_user_id: i64,
    menu_id: i64,
    payload: UpdateMenuStatusRequest,
) -> AppResult<MenuStatusMutationData> {
    let status = validate_status(payload.status)?;
    let mut tx = state.db.begin().await?;
    let current_menu = find_menu_identity(&mut tx, menu_id)
        .await?
        .ok_or_else(|| AppError::not_found("menu not found"))?;

    sqlx::query(
        r#"
        UPDATE sys_menu
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(menu_id)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    sync_menu_permission_status(
        &mut tx,
        operator_user_id,
        current_menu.permission_code.as_deref(),
        status,
    )
    .await?;

    tx.commit().await?;

    Ok(MenuStatusMutationData {
        id: current_menu.id,
        menu_name: current_menu.menu_name,
        status,
    })
}

pub async fn delete_menu(
    state: &AppState,
    operator_user_id: i64,
    menu_id: i64,
) -> AppResult<MenuMutationData> {
    let mut tx = state.db.begin().await?;
    let current_menu = find_menu_identity(&mut tx, menu_id)
        .await?
        .ok_or_else(|| AppError::not_found("menu not found"))?;

    ensure_menu_has_no_children(&mut tx, menu_id).await?;
    ensure_menu_permission_not_assigned(&mut tx, current_menu.permission_code.as_deref()).await?;

    sqlx::query(
        r#"
        UPDATE sys_menu
        SET is_deleted = TRUE,
            updated_at = NOW(),
            updated_by = $2
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(menu_id)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    deactivate_menu_permission(
        &mut tx,
        operator_user_id,
        current_menu.permission_code.as_deref(),
    )
    .await?;

    tx.commit().await?;

    Ok(MenuMutationData {
        id: current_menu.id,
        menu_name: current_menu.menu_name,
        menu_type: current_menu.menu_type,
    })
}

fn build_menu_tree(rows: Vec<MenuRow>) -> Vec<MenuTreeItem> {
    let existing_ids = rows.iter().map(|row| row.id).collect::<HashSet<_>>();
    let mut children_by_parent: HashMap<i64, Vec<MenuRow>> = HashMap::new();
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
        .flat_map(|parent_id| build_menu_nodes(parent_id, &mut children_by_parent))
        .collect()
}

fn build_menu_nodes(
    parent_id: i64,
    children_by_parent: &mut HashMap<i64, Vec<MenuRow>>,
) -> Vec<MenuTreeItem> {
    let Some(mut rows) = children_by_parent.remove(&parent_id) else {
        return Vec::new();
    };

    rows.sort_by_key(|row| (row.sort_no, row.id));
    rows.into_iter()
        .map(|row| {
            let children = build_menu_nodes(row.id, children_by_parent);
            map_menu_item(row, children)
        })
        .collect()
}

fn map_menu_item(row: MenuRow, children: Vec<MenuTreeItem>) -> MenuTreeItem {
    MenuTreeItem {
        id: row.id,
        parent_id: row.parent_id,
        menu_name: row.menu_name,
        menu_type: row.menu_type,
        route_name: row.route_name,
        route_path: row.route_path,
        component_path: row.component_path,
        permission_code: row.permission_code,
        icon: row.icon,
        sort_no: row.sort_no,
        visible: row.visible,
        keep_alive: row.keep_alive,
        is_external: row.is_external,
        status: row.status,
        remark: row.remark,
        created_at: row.created_at,
        updated_at: row.updated_at,
        children,
    }
}

async fn find_menu_row(state: &AppState, menu_id: i64) -> AppResult<Option<MenuRow>> {
    sqlx::query_as::<_, MenuRow>(
        r#"
        SELECT
            id,
            parent_id,
            menu_name,
            menu_type,
            route_name,
            route_path,
            component_path,
            permission_code,
            icon,
            sort_no,
            visible,
            keep_alive,
            is_external,
            status,
            remark,
            TO_CHAR(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_menu
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(menu_id)
    .fetch_optional(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_menu_identity(
    tx: &mut Transaction<'_, Postgres>,
    menu_id: i64,
) -> AppResult<Option<MenuIdentityRow>> {
    sqlx::query_as::<_, MenuIdentityRow>(
        r#"
        SELECT
            id,
            menu_name,
            menu_type,
            permission_code
        FROM sys_menu
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(menu_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn find_menu_edit_row(
    tx: &mut Transaction<'_, Postgres>,
    menu_id: i64,
) -> AppResult<Option<MenuEditRow>> {
    sqlx::query_as::<_, MenuEditRow>(
        r#"
        SELECT
            id,
            parent_id,
            menu_name,
            menu_type,
            route_name,
            route_path,
            component_path,
            permission_code,
            icon,
            sort_no,
            visible,
            keep_alive,
            is_external,
            status,
            remark
        FROM sys_menu
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(menu_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn next_menu_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 21000) + 10 AS next_id
        FROM sys_menu
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn next_permission_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 10000) + 1 AS next_id
        FROM sys_permission
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn sync_menu_permission(
    tx: &mut Transaction<'_, Postgres>,
    operator_user_id: i64,
    input: &MenuInput,
) -> AppResult<()> {
    let Some(permission_code) = input.permission_code.as_deref() else {
        return Ok(());
    };
    let Some(permission_type) = menu_type_to_permission_type(&input.menu_type) else {
        return Ok(());
    };

    let permission_id = next_permission_id(tx).await?;

    sqlx::query(
        r#"
        INSERT INTO sys_permission (
            id,
            permission_name,
            permission_code,
            permission_type,
            http_method,
            api_path,
            status,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, NULL, NULL, $5, NOW(), NOW(), $6, $6, FALSE, $7
        )
        ON CONFLICT (permission_code) DO UPDATE SET
            permission_name = EXCLUDED.permission_name,
            permission_type = EXCLUDED.permission_type,
            http_method = COALESCE(EXCLUDED.http_method, sys_permission.http_method),
            api_path = COALESCE(EXCLUDED.api_path, sys_permission.api_path),
            status = EXCLUDED.status,
            updated_at = NOW(),
            updated_by = EXCLUDED.updated_by,
            is_deleted = FALSE,
            remark = COALESCE(EXCLUDED.remark, sys_permission.remark)
        "#,
    )
    .bind(permission_id)
    .bind(&input.menu_name)
    .bind(permission_code)
    .bind(permission_type)
    .bind(input.status)
    .bind(operator_user_id)
    .bind(&input.remark)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn sync_menu_permission_status(
    tx: &mut Transaction<'_, Postgres>,
    operator_user_id: i64,
    permission_code: Option<&str>,
    status: i16,
) -> AppResult<()> {
    let Some(permission_code) = permission_code else {
        return Ok(());
    };

    sqlx::query(
        r#"
        UPDATE sys_permission
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE permission_code = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(permission_code)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn deactivate_menu_permission(
    tx: &mut Transaction<'_, Postgres>,
    operator_user_id: i64,
    permission_code: Option<&str>,
) -> AppResult<()> {
    let Some(permission_code) = permission_code else {
        return Ok(());
    };

    sqlx::query(
        r#"
        UPDATE sys_permission
        SET status = 0,
            is_deleted = TRUE,
            updated_at = NOW(),
            updated_by = $2
        WHERE permission_code = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(permission_code)
    .bind(operator_user_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn ensure_parent_exists(tx: &mut Transaction<'_, Postgres>, parent_id: i64) -> AppResult<()> {
    if parent_id == 0 {
        return Ok(());
    }

    let exists = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_menu
            WHERE id = $1
              AND is_deleted = FALSE
        ) AS exists_flag
        "#,
    )
    .bind(parent_id)
    .fetch_one(&mut **tx)
    .await?;

    if !exists.exists_flag {
        return Err(AppError::bad_request("parent menu does not exist"));
    }

    Ok(())
}

async fn ensure_parent_not_descendant(
    tx: &mut Transaction<'_, Postgres>,
    menu_id: i64,
    parent_id: i64,
) -> AppResult<()> {
    if parent_id == 0 {
        return Ok(());
    }

    if parent_id == menu_id {
        return Err(AppError::bad_request("parent menu cannot be self"));
    }

    let mut current_parent_id = parent_id;
    let mut visited = HashSet::new();
    while current_parent_id != 0 {
        if !visited.insert(current_parent_id) {
            return Err(AppError::bad_request("menu parent chain has a cycle"));
        }

        if current_parent_id == menu_id {
            return Err(AppError::bad_request(
                "parent menu cannot be a descendant of current menu",
            ));
        }

        let next_parent_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT parent_id
            FROM sys_menu
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

async fn ensure_unique_permission_code(
    tx: &mut Transaction<'_, Postgres>,
    permission_code: Option<&str>,
    exclude_menu_id: Option<i64>,
) -> AppResult<()> {
    let Some(permission_code) = permission_code else {
        return Ok(());
    };

    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_menu
            WHERE permission_code = $1
              AND is_deleted = FALSE
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(permission_code)
    .bind(exclude_menu_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.exists_flag {
        return Err(AppError::conflict("permission_code already exists"));
    }

    Ok(())
}

async fn ensure_menu_has_no_children(
    tx: &mut Transaction<'_, Postgres>,
    menu_id: i64,
) -> AppResult<()> {
    let row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_menu
        WHERE parent_id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(menu_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.total > 0 {
        return Err(AppError::conflict(
            "menu has child nodes and cannot be deleted",
        ));
    }

    Ok(())
}

async fn ensure_menu_permission_not_assigned(
    tx: &mut Transaction<'_, Postgres>,
    permission_code: Option<&str>,
) -> AppResult<()> {
    let Some(permission_code) = permission_code else {
        return Ok(());
    };

    let row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_role_permission rp
        INNER JOIN sys_permission p ON p.id = rp.permission_id
        WHERE p.permission_code = $1
          AND p.is_deleted = FALSE
        "#,
    )
    .bind(permission_code)
    .fetch_one(&mut **tx)
    .await?;

    if row.total > 0 {
        return Err(AppError::conflict(
            "menu permission has role assignments and cannot be deleted",
        ));
    }

    Ok(())
}

fn normalize_create_input(payload: CreateMenuRequest) -> AppResult<MenuInput> {
    let menu_type = normalize_menu_type(payload.menu_type)?;
    let input = MenuInput {
        parent_id: validate_parent_id(payload.parent_id.unwrap_or(0))?,
        menu_name: require_non_empty(payload.menu_name, "menu_name")?,
        route_name: normalize_optional_field(payload.route_name),
        route_path: normalize_optional_field(payload.route_path),
        component_path: normalize_optional_field(payload.component_path),
        permission_code: normalize_optional_field(payload.permission_code),
        icon: normalize_optional_field(payload.icon),
        sort_no: payload.sort_no.unwrap_or(0),
        visible: payload.visible.unwrap_or(true),
        keep_alive: payload.keep_alive.unwrap_or(false),
        is_external: payload.is_external.unwrap_or(false),
        status: validate_status(payload.status.unwrap_or(1))?,
        remark: normalize_optional_field(payload.remark),
        menu_type,
    };

    validate_menu_input(input)
}

fn normalize_update_input(
    current_menu: &MenuEditRow,
    payload: UpdateMenuRequest,
) -> AppResult<MenuInput> {
    let menu_type = payload
        .menu_type
        .map(normalize_menu_type)
        .transpose()?
        .unwrap_or_else(|| current_menu.menu_type.clone());

    let input = MenuInput {
        parent_id: payload
            .parent_id
            .map(validate_parent_id)
            .transpose()?
            .unwrap_or(current_menu.parent_id),
        menu_name: payload
            .menu_name
            .map(|value| require_non_empty(value, "menu_name"))
            .transpose()?
            .unwrap_or_else(|| current_menu.menu_name.clone()),
        route_path: payload
            .route_path
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.route_path.clone()),
        component_path: payload
            .component_path
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.component_path.clone()),
        permission_code: payload
            .permission_code
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.permission_code.clone()),
        route_name: payload
            .route_name
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.route_name.clone()),
        icon: payload
            .icon
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.icon.clone()),
        sort_no: payload.sort_no.unwrap_or(current_menu.sort_no),
        visible: payload.visible.unwrap_or(current_menu.visible),
        keep_alive: payload.keep_alive.unwrap_or(current_menu.keep_alive),
        is_external: payload.is_external.unwrap_or(current_menu.is_external),
        status: validate_status(payload.status.unwrap_or(current_menu.status))?,
        remark: payload
            .remark
            .map(|value| normalize_optional_field(Some(value)))
            .unwrap_or_else(|| current_menu.remark.clone()),
        menu_type,
    };

    validate_menu_input(input)
}

fn validate_menu_input(input: MenuInput) -> AppResult<MenuInput> {
    if matches!(input.menu_type.as_str(), "button" | "api") && input.permission_code.is_none() {
        return Err(AppError::bad_request(
            "permission_code is required for button or api menu",
        ));
    }

    if input.menu_type == "menu" && input.route_path.is_none() {
        return Err(AppError::bad_request(
            "route_path is required for menu type",
        ));
    }

    if let Some(route_path) = input.route_path.as_deref()
        && !route_path.starts_with('/')
    {
        return Err(AppError::bad_request("route_path must start with /"));
    }

    Ok(input)
}

fn normalize_menu_type(value: impl AsRef<str>) -> AppResult<String> {
    let value = value.as_ref().trim().to_ascii_lowercase();
    match value.as_str() {
        "catalog" | "menu" | "button" | "api" => Ok(value),
        _ => Err(AppError::bad_request(
            "menu_type must be one of catalog, menu, button, api",
        )),
    }
}

fn menu_type_to_permission_type(menu_type: &str) -> Option<&'static str> {
    match menu_type {
        "menu" => Some("menu"),
        "button" => Some("button"),
        "api" => Some("api"),
        _ => None,
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
