use sqlx::{FromRow, Postgres, Transaction};
use std::collections::HashMap;

use crate::{
    common::{
        error::{AppError, AppResult},
        password,
    },
    modules::system_user::dto::{
        CreateUserRequest, PaginationMeta, UpdateUserRequest, UpdateUserRolesRequest,
        UpdateUserStatusRequest, UserDeptSummary, UserDetailData, UserListData, UserListItem,
        UserListQuery, UserMutationData, UserPostSummary, UserRoleSummary, UserStatusMutationData,
    },
    state::AppState,
};

#[derive(Debug, Clone, FromRow)]
struct UserListRow {
    id: i64,
    username: String,
    nickname: String,
    real_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    status: i16,
    is_super_admin: bool,
    dept_id: Option<i64>,
    dept_name: Option<String>,
    last_login_at: Option<String>,
    created_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct UserDetailRow {
    id: i64,
    username: String,
    nickname: String,
    real_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    gender: Option<i16>,
    status: i16,
    is_super_admin: bool,
    remark: Option<String>,
    dept_id: Option<i64>,
    dept_name: Option<String>,
    last_login_at: Option<String>,
    last_login_ip: Option<String>,
    password_updated_at: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct UserRoleRow {
    user_id: i64,
    role_id: i64,
    role_code: String,
    role_name: String,
}

#[derive(Debug, Clone, FromRow)]
struct UserPostRow {
    user_id: i64,
    post_id: i64,
    post_code: String,
    post_name: String,
}

#[derive(Debug, Clone, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, Clone, FromRow)]
struct UserIdentityRow {
    id: i64,
    username: String,
    nickname: String,
    is_super_admin: bool,
}

#[derive(Debug, Clone, FromRow)]
struct RoleTargetRow {
    id: i64,
    role_code: String,
}

#[derive(Debug, Clone, FromRow)]
struct ExistsRow {
    exists_flag: bool,
}

pub async fn list_users(state: &AppState, query: UserListQuery) -> AppResult<UserListData> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let offset = ((page - 1) * page_size) as i64;
    let keyword = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{value}%"));

    let total_row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_user u
        WHERE u.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR u.username ILIKE $1
                OR u.nickname ILIKE $1
                OR COALESCE(u.real_name, '') ILIKE $1
                OR COALESCE(u.mobile, '') ILIKE $1)
          AND ($2::BIGINT IS NULL OR u.dept_id = $2)
          AND ($3::SMALLINT IS NULL OR u.status = $3)
        "#,
    )
    .bind(keyword.as_deref())
    .bind(query.dept_id)
    .bind(query.status)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, UserListRow>(
        r#"
        SELECT
            u.id,
            u.username,
            u.nickname,
            u.real_name,
            u.mobile,
            u.email,
            u.status,
            u.is_super_admin,
            u.dept_id,
            d.dept_name,
            TO_CHAR(u.last_login_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS last_login_at,
            TO_CHAR(u.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM sys_user u
        LEFT JOIN sys_dept d ON d.id = u.dept_id
        WHERE u.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR u.username ILIKE $1
                OR u.nickname ILIKE $1
                OR COALESCE(u.real_name, '') ILIKE $1
                OR COALESCE(u.mobile, '') ILIKE $1)
          AND ($2::BIGINT IS NULL OR u.dept_id = $2)
          AND ($3::SMALLINT IS NULL OR u.status = $3)
        ORDER BY u.id
        OFFSET $4
        LIMIT $5
        "#,
    )
    .bind(keyword.as_deref())
    .bind(query.dept_id)
    .bind(query.status)
    .bind(offset)
    .bind(page_size as i64)
    .fetch_all(&state.db)
    .await?;

    let user_ids: Vec<i64> = rows.iter().map(|row| row.id).collect();
    let roles_by_user = find_roles_by_user_ids(state, &user_ids).await?;
    let posts_by_user = find_posts_by_user_ids(state, &user_ids).await?;

    let items = rows
        .into_iter()
        .map(|row| UserListItem {
            id: row.id,
            username: row.username,
            nickname: row.nickname,
            real_name: row.real_name,
            mobile: row.mobile,
            email: row.email,
            status: row.status,
            is_super_admin: row.is_super_admin,
            dept: map_dept(row.dept_id, row.dept_name),
            roles: roles_by_user.get(&row.id).cloned().unwrap_or_default(),
            posts: posts_by_user.get(&row.id).cloned().unwrap_or_default(),
            last_login_at: row.last_login_at,
            created_at: row.created_at,
        })
        .collect();

    Ok(UserListData {
        items,
        pagination: PaginationMeta {
            page,
            page_size,
            total: total_row.total.max(0) as u64,
        },
    })
}

pub async fn get_user_detail(state: &AppState, user_id: i64) -> AppResult<UserDetailData> {
    let row = sqlx::query_as::<_, UserDetailRow>(
        r#"
        SELECT
            u.id,
            u.username,
            u.nickname,
            u.real_name,
            u.mobile,
            u.email,
            u.avatar_url,
            u.gender,
            u.status,
            u.is_super_admin,
            u.remark,
            u.dept_id,
            d.dept_name,
            TO_CHAR(u.last_login_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS last_login_at,
            u.last_login_ip,
            TO_CHAR(u.password_updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS password_updated_at,
            TO_CHAR(u.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(u.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_user u
        LEFT JOIN sys_dept d ON d.id = u.dept_id
        WHERE u.id = $1
          AND u.is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("user not found"))?;

    let roles_by_user = find_roles_by_user_ids(state, &[row.id]).await?;
    let posts_by_user = find_posts_by_user_ids(state, &[row.id]).await?;

    Ok(UserDetailData {
        id: row.id,
        username: row.username,
        nickname: row.nickname,
        real_name: row.real_name,
        mobile: row.mobile,
        email: row.email,
        avatar_url: row.avatar_url,
        gender: row.gender,
        status: row.status,
        is_super_admin: row.is_super_admin,
        remark: row.remark,
        dept: map_dept(row.dept_id, row.dept_name),
        roles: roles_by_user.get(&row.id).cloned().unwrap_or_default(),
        posts: posts_by_user.get(&row.id).cloned().unwrap_or_default(),
        last_login_at: row.last_login_at,
        last_login_ip: row.last_login_ip,
        password_updated_at: row.password_updated_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
}

pub async fn create_user(
    state: &AppState,
    operator_user_id: i64,
    payload: CreateUserRequest,
) -> AppResult<UserMutationData> {
    let username = require_non_empty(payload.username, "username")?;
    let password_text = require_non_empty(payload.password, "password")?;
    let nickname = require_non_empty(payload.nickname, "nickname")?;
    let real_name = normalize_optional_field(payload.real_name, "real_name")?;
    let mobile = normalize_optional_field(payload.mobile, "mobile")?;
    let email = normalize_optional_field(payload.email, "email")?;
    let remark = normalize_optional_field(payload.remark, "remark")?;
    let status = validate_status(payload.status.unwrap_or(1))?;
    let role_ids = normalize_role_ids(payload.role_ids.unwrap_or_default())?;

    let mut tx = state.db.begin().await?;

    ensure_unique_user_fields(
        &mut tx,
        &username,
        mobile.as_deref(),
        email.as_deref(),
        None,
    )
    .await?;
    ensure_dept_exists(&mut tx, payload.dept_id).await?;
    let role_targets = find_assignable_roles_by_ids(&mut tx, &role_ids).await?;
    ensure_requested_roles_exist(&role_ids, &role_targets)?;

    let new_user_id = next_user_id(&mut tx).await?;
    let password_hash = password::hash_password(&password_text)?;

    sqlx::query(
        r#"
        INSERT INTO sys_user (
            id,
            username,
            password_hash,
            nickname,
            real_name,
            mobile,
            email,
            gender,
            dept_id,
            status,
            is_super_admin,
            password_updated_at,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, FALSE, NOW(), NOW(), NOW(), $11, $11, FALSE, $12
        )
        "#,
    )
    .bind(new_user_id)
    .bind(&username)
    .bind(&password_hash)
    .bind(&nickname)
    .bind(real_name)
    .bind(mobile)
    .bind(email)
    .bind(payload.gender)
    .bind(payload.dept_id)
    .bind(status)
    .bind(operator_user_id)
    .bind(remark)
    .execute(&mut *tx)
    .await?;

    replace_user_roles(&mut tx, new_user_id, &role_targets, operator_user_id).await?;

    tx.commit().await?;

    Ok(UserMutationData {
        id: new_user_id,
        username,
    })
}

pub async fn update_user(
    state: &AppState,
    operator_user_id: i64,
    user_id: i64,
    payload: UpdateUserRequest,
) -> AppResult<UserMutationData> {
    let mut tx = state.db.begin().await?;
    let current_user = find_user_identity(&mut tx, user_id)
        .await?
        .ok_or_else(|| AppError::not_found("user not found"))?;

    let nickname = resolve_optional_update(payload.nickname, &current_user.nickname, "nickname")?;
    let real_name = normalize_optional_field(payload.real_name, "real_name")?;
    let mobile = normalize_optional_field(payload.mobile, "mobile")?;
    let email = normalize_optional_field(payload.email, "email")?;
    let remark = normalize_optional_field(payload.remark, "remark")?;
    let role_ids = payload.role_ids.map(normalize_role_ids).transpose()?;

    if nickname.is_none()
        && real_name.is_none()
        && mobile.is_none()
        && email.is_none()
        && payload.gender.is_none()
        && payload.dept_id.is_none()
        && role_ids.is_none()
        && remark.is_none()
    {
        return Err(AppError::bad_request(
            "at least one updatable field is required",
        ));
    }

    ensure_unique_user_fields(
        &mut tx,
        &current_user.username,
        mobile.as_deref(),
        email.as_deref(),
        Some(user_id),
    )
    .await?;
    ensure_dept_exists(&mut tx, payload.dept_id).await?;
    let role_targets = if let Some(role_ids) = role_ids.as_ref() {
        let role_targets = find_assignable_roles_by_ids(&mut tx, role_ids).await?;
        ensure_requested_roles_exist(role_ids, &role_targets)?;
        ensure_super_admin_role_integrity(&current_user, &role_targets)?;
        Some(role_targets)
    } else {
        None
    };

    sqlx::query(
        r#"
        UPDATE sys_user
        SET nickname = COALESCE($2, nickname),
            real_name = COALESCE($3, real_name),
            mobile = COALESCE($4, mobile),
            email = COALESCE($5, email),
            gender = COALESCE($6, gender),
            dept_id = COALESCE($7, dept_id),
            remark = COALESCE($8, remark),
            updated_at = NOW(),
            updated_by = $9
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(user_id)
    .bind(nickname)
    .bind(real_name)
    .bind(mobile)
    .bind(email)
    .bind(payload.gender)
    .bind(payload.dept_id)
    .bind(remark)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    if let Some(role_targets) = role_targets.as_ref() {
        replace_user_roles(&mut tx, user_id, role_targets, operator_user_id).await?;
    }

    tx.commit().await?;

    Ok(UserMutationData {
        id: current_user.id,
        username: current_user.username,
    })
}

pub async fn update_user_status(
    state: &AppState,
    operator_user_id: i64,
    user_id: i64,
    payload: UpdateUserStatusRequest,
) -> AppResult<UserStatusMutationData> {
    let status = validate_status(payload.status)?;
    let mut tx = state.db.begin().await?;
    let current_user = find_user_identity(&mut tx, user_id)
        .await?
        .ok_or_else(|| AppError::not_found("user not found"))?;

    if current_user.is_super_admin && status == 0 {
        return Err(AppError::forbidden("super admin user cannot be disabled"));
    }

    if current_user.id == operator_user_id && status == 0 {
        return Err(AppError::forbidden("current user cannot disable self"));
    }

    sqlx::query(
        r#"
        UPDATE sys_user
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(user_id)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(UserStatusMutationData {
        id: current_user.id,
        username: current_user.username,
        status,
    })
}

pub async fn update_user_roles(
    state: &AppState,
    operator_user_id: i64,
    user_id: i64,
    payload: UpdateUserRolesRequest,
) -> AppResult<UserMutationData> {
    let role_ids = normalize_role_ids(payload.role_ids)?;
    let mut tx = state.db.begin().await?;
    let current_user = find_user_identity(&mut tx, user_id)
        .await?
        .ok_or_else(|| AppError::not_found("user not found"))?;
    let role_targets = find_assignable_roles_by_ids(&mut tx, &role_ids).await?;

    ensure_requested_roles_exist(&role_ids, &role_targets)?;
    ensure_super_admin_role_integrity(&current_user, &role_targets)?;
    replace_user_roles(&mut tx, user_id, &role_targets, operator_user_id).await?;

    sqlx::query(
        r#"
        UPDATE sys_user
        SET updated_at = NOW(),
            updated_by = $2
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(user_id)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(UserMutationData {
        id: current_user.id,
        username: current_user.username,
    })
}

async fn find_roles_by_user_ids(
    state: &AppState,
    user_ids: &[i64],
) -> AppResult<HashMap<i64, Vec<UserRoleSummary>>> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = sqlx::query_as::<_, UserRoleRow>(
        r#"
        SELECT
            ur.user_id,
            r.id AS role_id,
            r.role_code,
            r.role_name
        FROM sys_user_role ur
        INNER JOIN sys_role r ON r.id = ur.role_id
        WHERE ur.user_id = ANY($1)
          AND r.is_deleted = FALSE
        ORDER BY r.role_sort, r.id
        "#,
    )
    .bind(user_ids)
    .fetch_all(&state.db)
    .await?;

    let mut map = HashMap::new();
    for row in rows {
        map.entry(row.user_id)
            .or_insert_with(Vec::new)
            .push(UserRoleSummary {
                id: row.role_id,
                code: row.role_code,
                name: row.role_name,
            });
    }

    Ok(map)
}

async fn find_posts_by_user_ids(
    state: &AppState,
    user_ids: &[i64],
) -> AppResult<HashMap<i64, Vec<UserPostSummary>>> {
    if user_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = sqlx::query_as::<_, UserPostRow>(
        r#"
        SELECT
            up.user_id,
            p.id AS post_id,
            p.post_code,
            p.post_name
        FROM sys_user_post up
        INNER JOIN sys_post p ON p.id = up.post_id
        WHERE up.user_id = ANY($1)
          AND p.is_deleted = FALSE
        ORDER BY p.sort_no, p.id
        "#,
    )
    .bind(user_ids)
    .fetch_all(&state.db)
    .await?;

    let mut map = HashMap::new();
    for row in rows {
        map.entry(row.user_id)
            .or_insert_with(Vec::new)
            .push(UserPostSummary {
                id: row.post_id,
                code: row.post_code,
                name: row.post_name,
            });
    }

    Ok(map)
}

fn map_dept(dept_id: Option<i64>, dept_name: Option<String>) -> Option<UserDeptSummary> {
    match (dept_id, dept_name) {
        (Some(id), Some(name)) => Some(UserDeptSummary { id, name }),
        _ => None,
    }
}

async fn next_user_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    #[derive(Debug, FromRow)]
    struct NextIdRow {
        next_id: i64,
    }

    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 1000) + 10 AS next_id
        FROM sys_user
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn next_user_role_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    #[derive(Debug, FromRow)]
    struct NextIdRow {
        next_id: i64,
    }

    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 30000) + 10 AS next_id
        FROM sys_user_role
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn find_assignable_roles_by_ids(
    tx: &mut Transaction<'_, Postgres>,
    role_ids: &[i64],
) -> AppResult<Vec<RoleTargetRow>> {
    if role_ids.is_empty() {
        return Ok(Vec::new());
    }

    sqlx::query_as::<_, RoleTargetRow>(
        r#"
        SELECT
            id,
            role_code
        FROM sys_role
        WHERE id = ANY($1)
          AND is_deleted = FALSE
          AND status = 1
        ORDER BY role_sort, id
        "#,
    )
    .bind(role_ids)
    .fetch_all(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn replace_user_roles(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    role_targets: &[RoleTargetRow],
    operator_user_id: i64,
) -> AppResult<()> {
    sqlx::query(
        r#"
        DELETE FROM sys_user_role
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    if role_targets.is_empty() {
        return Ok(());
    }

    let mut next_id = next_user_role_id(tx).await?;
    for role in role_targets {
        sqlx::query(
            r#"
            INSERT INTO sys_user_role (
                id,
                user_id,
                role_id,
                created_at,
                created_by
            ) VALUES ($1, $2, $3, NOW(), $4)
            "#,
        )
        .bind(next_id)
        .bind(user_id)
        .bind(role.id)
        .bind(operator_user_id)
        .execute(&mut **tx)
        .await?;

        next_id += 10;
    }

    Ok(())
}

fn ensure_requested_roles_exist(
    requested_role_ids: &[i64],
    role_targets: &[RoleTargetRow],
) -> AppResult<()> {
    if requested_role_ids.len() == role_targets.len() {
        return Ok(());
    }

    let existing_ids = role_targets
        .iter()
        .map(|role| role.id)
        .collect::<std::collections::BTreeSet<_>>();
    let invalid_ids = requested_role_ids
        .iter()
        .filter(|role_id| !existing_ids.contains(role_id))
        .map(|role_id| role_id.to_string())
        .collect::<Vec<_>>();

    Err(AppError::conflict(format!(
        "role_id does not exist or is disabled: {}",
        invalid_ids.join(", ")
    )))
}

fn ensure_super_admin_role_integrity(
    current_user: &UserIdentityRow,
    role_targets: &[RoleTargetRow],
) -> AppResult<()> {
    if !current_user.is_super_admin {
        return Ok(());
    }

    let retains_super_admin_role = role_targets
        .iter()
        .any(|role| role.role_code == "super_admin");

    if !retains_super_admin_role {
        return Err(AppError::forbidden(
            "super admin user must retain the super_admin role",
        ));
    }

    Ok(())
}

async fn ensure_unique_user_fields(
    tx: &mut Transaction<'_, Postgres>,
    username: &str,
    mobile: Option<&str>,
    email: Option<&str>,
    exclude_user_id: Option<i64>,
) -> AppResult<()> {
    if exists_user_by_username(tx, username, exclude_user_id).await? {
        return Err(AppError::conflict("username already exists"));
    }

    if let Some(mobile) = mobile
        && exists_user_by_mobile(tx, mobile, exclude_user_id).await?
    {
        return Err(AppError::conflict("mobile already exists"));
    }

    if let Some(email) = email
        && exists_user_by_email(tx, email, exclude_user_id).await?
    {
        return Err(AppError::conflict("email already exists"));
    }

    Ok(())
}

async fn ensure_dept_exists(
    tx: &mut Transaction<'_, Postgres>,
    dept_id: Option<i64>,
) -> AppResult<()> {
    let Some(dept_id) = dept_id else {
        return Ok(());
    };

    let exists = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_dept
            WHERE id = $1
              AND is_deleted = FALSE
        ) AS exists_flag
        "#,
    )
    .bind(dept_id)
    .fetch_one(&mut **tx)
    .await?;

    if !exists.exists_flag {
        return Err(AppError::bad_request("dept_id does not exist"));
    }

    Ok(())
}

async fn find_user_identity(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> AppResult<Option<UserIdentityRow>> {
    sqlx::query_as::<_, UserIdentityRow>(
        r#"
        SELECT id, username, nickname, is_super_admin
        FROM sys_user
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn exists_user_by_username(
    tx: &mut Transaction<'_, Postgres>,
    username: &str,
    exclude_user_id: Option<i64>,
) -> AppResult<bool> {
    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_user
            WHERE username = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(username)
    .bind(exclude_user_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.exists_flag)
}

async fn exists_user_by_mobile(
    tx: &mut Transaction<'_, Postgres>,
    mobile: &str,
    exclude_user_id: Option<i64>,
) -> AppResult<bool> {
    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_user
            WHERE mobile = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(mobile)
    .bind(exclude_user_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.exists_flag)
}

async fn exists_user_by_email(
    tx: &mut Transaction<'_, Postgres>,
    email: &str,
    exclude_user_id: Option<i64>,
) -> AppResult<bool> {
    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_user
            WHERE email = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(email)
    .bind(exclude_user_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.exists_flag)
}

fn require_non_empty(value: String, field_name: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request(format!("{field_name} is required")));
    }

    Ok(trimmed.to_string())
}

fn normalize_optional_field(value: Option<String>, field_name: &str) -> AppResult<Option<String>> {
    match value {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(AppError::bad_request(format!(
                    "{field_name} cannot be empty"
                )));
            }
            Ok(Some(trimmed.to_string()))
        }
        None => Ok(None),
    }
}

fn resolve_optional_update(
    value: Option<String>,
    current_value: &str,
    field_name: &str,
) -> AppResult<Option<String>> {
    match value {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(AppError::bad_request(format!(
                    "{field_name} cannot be empty"
                )));
            }

            if trimmed == current_value {
                return Ok(None);
            }

            Ok(Some(trimmed.to_string()))
        }
        None => Ok(None),
    }
}

fn validate_status(status: i16) -> AppResult<i16> {
    if status != 0 && status != 1 {
        return Err(AppError::bad_request("status must be 0 or 1"));
    }

    Ok(status)
}

fn normalize_role_ids(role_ids: Vec<i64>) -> AppResult<Vec<i64>> {
    let mut normalized = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for role_id in role_ids {
        if role_id <= 0 {
            return Err(AppError::bad_request(
                "role_ids must contain positive integers",
            ));
        }

        if seen.insert(role_id) {
            normalized.push(role_id);
        }
    }

    Ok(normalized)
}
