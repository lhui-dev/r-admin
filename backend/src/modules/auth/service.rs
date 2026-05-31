use sqlx::FromRow;
use tracing::warn;

use crate::{
    common::{
        error::{AppError, AppResult},
        jwt, password,
    },
    modules::auth::dto::{AuthUserProfile, CurrentUserResponse, LoginResponse},
    state::AppState,
};

#[derive(Debug, FromRow)]
struct UserAuthRow {
    id: i64,
    username: String,
    password_hash: String,
    nickname: String,
    real_name: Option<String>,
    status: i16,
    is_super_admin: bool,
    is_deleted: bool,
}

#[derive(Debug, FromRow)]
struct RoleCodeRow {
    role_code: String,
}

#[derive(Debug, FromRow)]
struct PermissionCodeRow {
    permission_code: String,
}

pub async fn login(
    state: &AppState,
    username: &str,
    password_text: &str,
) -> AppResult<LoginResponse> {
    let username = username.trim();

    if username.is_empty() {
        return Err(AppError::bad_request("username is required"));
    }

    if password_text.trim().is_empty() {
        return Err(AppError::bad_request("password is required"));
    }

    let user = find_user_by_username(state, username)
        .await?
        .ok_or_else(|| AppError::unauthorized("username or password is incorrect"))?;

    if user.is_deleted {
        return Err(AppError::unauthorized("username or password is incorrect"));
    }

    if user.status != 1 {
        // Return the same outward-facing auth failure to avoid leaking whether
        // a specific username exists but is currently disabled.
        warn!(username = %user.username, "disabled user login attempt rejected");
        return Err(AppError::unauthorized("username or password is incorrect"));
    }

    let password_ok = password::verify_password(password_text, &user.password_hash)?;
    if !password_ok {
        return Err(AppError::unauthorized("username or password is incorrect"));
    }

    update_last_login(state, user.id).await?;

    Ok(LoginResponse {
        access_token: jwt::generate_access_token(
            user.id,
            &user.username,
            user.is_super_admin,
            &state.jwt.secret,
            state.jwt.expires_in,
        )?,
        token_type: "Bearer",
        expires_in: state.jwt.expires_in,
        user: map_profile(user),
    })
}

pub async fn current_user(state: &AppState, user_id: i64) -> AppResult<CurrentUserResponse> {
    let user = find_user_by_id(state, user_id)
        .await?
        .ok_or_else(|| AppError::not_found("current user not found"))?;

    if user.is_deleted {
        return Err(AppError::not_found("current user not found"));
    }

    if user.status != 1 {
        return Err(AppError::forbidden("user account is disabled"));
    }

    // These two lookups are independent, so fetch them together to reduce the
    // latency of the current-user bootstrap request.
    let (roles, permissions) = tokio::try_join!(
        find_role_codes_by_user_id(state, user.id),
        find_permission_codes_by_user_id(state, user.id)
    )?;

    Ok(CurrentUserResponse {
        user: map_profile(user),
        roles,
        permissions,
    })
}

async fn find_user_by_username(state: &AppState, username: &str) -> AppResult<Option<UserAuthRow>> {
    sqlx::query_as::<_, UserAuthRow>(
        r#"
        SELECT id, username, password_hash, nickname, real_name, status, is_super_admin, is_deleted
        FROM sys_user
        WHERE username = $1
        LIMIT 1
        "#,
    )
    .bind(username)
    .fetch_optional(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_user_by_id(state: &AppState, user_id: i64) -> AppResult<Option<UserAuthRow>> {
    sqlx::query_as::<_, UserAuthRow>(
        r#"
        SELECT id, username, password_hash, nickname, real_name, status, is_super_admin, is_deleted
        FROM sys_user
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_role_codes_by_user_id(state: &AppState, user_id: i64) -> AppResult<Vec<String>> {
    sqlx::query_as::<_, RoleCodeRow>(
        r#"
        SELECT DISTINCT r.role_code
        FROM sys_role r
        INNER JOIN sys_user_role ur ON ur.role_id = r.id
        WHERE ur.user_id = $1
          AND r.is_deleted = FALSE
          AND r.status = 1
        ORDER BY r.role_code
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .map(|rows| rows.into_iter().map(|row| row.role_code).collect())
    .map_err(Into::into)
}

async fn find_permission_codes_by_user_id(
    state: &AppState,
    user_id: i64,
) -> AppResult<Vec<String>> {
    sqlx::query_as::<_, PermissionCodeRow>(
        r#"
        SELECT DISTINCT p.permission_code
        FROM sys_permission p
        INNER JOIN sys_role_permission rp ON rp.permission_id = p.id
        INNER JOIN sys_user_role ur ON ur.role_id = rp.role_id
        INNER JOIN sys_role r ON r.id = ur.role_id
        WHERE ur.user_id = $1
          AND p.is_deleted = FALSE
          AND p.status = 1
          AND r.is_deleted = FALSE
          AND r.status = 1
        ORDER BY p.permission_code
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .map(|rows| rows.into_iter().map(|row| row.permission_code).collect())
    .map_err(Into::into)
}

async fn update_last_login(state: &AppState, user_id: i64) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE sys_user
        SET last_login_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(())
}

fn map_profile(user: UserAuthRow) -> AuthUserProfile {
    AuthUserProfile {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        real_name: user.real_name,
        is_super_admin: user.is_super_admin,
    }
}
