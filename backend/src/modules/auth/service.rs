use sqlx::FromRow;
use std::collections::HashMap;
use tracing::warn;

use crate::{
    common::{
        error::{AppError, AppResult},
        jwt, password,
    },
    modules::auth::dto::{
        AuthUserProfile, CurrentMenuItem, CurrentMenusResponse, CurrentUserResponse, LoginResponse,
    },
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

#[derive(Debug, Clone, FromRow)]
struct MenuRow {
    id: i64,
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    route_name: Option<String>,
    route_path: Option<String>,
    permission_code: Option<String>,
    icon: Option<String>,
    visible: bool,
    sort_no: i32,
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
    let user = require_active_user(state, user_id).await?;

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

pub async fn current_menus(state: &AppState, user_id: i64) -> AppResult<CurrentMenusResponse> {
    let user = require_active_user(state, user_id).await?;

    let menu_rows = if user.is_super_admin {
        find_all_visible_menu_rows(state).await?
    } else {
        find_visible_menu_rows_by_user_id(state, user.id).await?
    };

    Ok(CurrentMenusResponse {
        menus: build_menu_tree(menu_rows),
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

async fn find_all_visible_menu_rows(state: &AppState) -> AppResult<Vec<MenuRow>> {
    sqlx::query_as::<_, MenuRow>(
        r#"
        SELECT
            id,
            parent_id,
            menu_name,
            menu_type,
            route_name,
            route_path,
            permission_code,
            icon,
            visible,
            sort_no
        FROM sys_menu
        WHERE is_deleted = FALSE
          AND status = 1
          AND menu_type IN ('catalog', 'menu')
        ORDER BY parent_id, sort_no, id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_visible_menu_rows_by_user_id(
    state: &AppState,
    user_id: i64,
) -> AppResult<Vec<MenuRow>> {
    sqlx::query_as::<_, MenuRow>(
        r#"
        SELECT DISTINCT
            m.id,
            m.parent_id,
            m.menu_name,
            m.menu_type,
            m.route_name,
            m.route_path,
            m.permission_code,
            m.icon,
            m.visible,
            m.sort_no
        FROM sys_menu m
        WHERE m.is_deleted = FALSE
          AND m.status = 1
          AND m.menu_type IN ('catalog', 'menu')
          AND (
                m.permission_code IS NULL
                OR EXISTS (
                    SELECT 1
                    FROM sys_user_role ur
                    INNER JOIN sys_role r ON r.id = ur.role_id
                    INNER JOIN sys_role_permission rp ON rp.role_id = ur.role_id
                    INNER JOIN sys_permission p ON p.id = rp.permission_id
                    WHERE ur.user_id = $1
                      AND r.is_deleted = FALSE
                      AND r.status = 1
                      AND p.is_deleted = FALSE
                      AND p.status = 1
                      AND p.permission_code = m.permission_code
                )
              )
        ORDER BY m.parent_id, m.sort_no, m.id
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
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

async fn require_active_user(state: &AppState, user_id: i64) -> AppResult<UserAuthRow> {
    let user = find_user_by_id(state, user_id)
        .await?
        .ok_or_else(|| AppError::not_found("current user not found"))?;

    if user.is_deleted {
        return Err(AppError::not_found("current user not found"));
    }

    if user.status != 1 {
        return Err(AppError::forbidden("user account is disabled"));
    }

    Ok(user)
}

fn build_menu_tree(rows: Vec<MenuRow>) -> Vec<CurrentMenuItem> {
    let mut children_by_parent: HashMap<i64, Vec<MenuRow>> = HashMap::new();

    for row in rows {
        children_by_parent
            .entry(row.parent_id)
            .or_default()
            .push(row);
    }

    for children in children_by_parent.values_mut() {
        children.sort_by_key(|row| (row.sort_no, row.id));
    }

    build_menu_nodes(0, &children_by_parent)
}

fn build_menu_nodes(
    parent_id: i64,
    children_by_parent: &HashMap<i64, Vec<MenuRow>>,
) -> Vec<CurrentMenuItem> {
    children_by_parent
        .get(&parent_id)
        .into_iter()
        .flatten()
        .filter_map(|row| {
            let children = build_menu_nodes(row.id, children_by_parent);
            let path = normalize_menu_path(row);

            if !should_include_menu_node(row, &children, path.as_deref()) {
                return None;
            }

            Some(CurrentMenuItem {
                id: row.id.to_string(),
                name: row
                    .route_name
                    .clone()
                    .unwrap_or_else(|| format!("menu-{}", row.id)),
                title: row.menu_name.clone(),
                path,
                icon: normalize_menu_icon(row),
                permission: row.permission_code.clone(),
                hidden: !row.visible,
                children,
            })
        })
        .collect()
}

fn should_include_menu_node(
    row: &MenuRow,
    children: &[CurrentMenuItem],
    path: Option<&str>,
) -> bool {
    if row.menu_type == "catalog" {
        return !children.is_empty();
    }

    path.is_some()
}

fn normalize_menu_path(row: &MenuRow) -> Option<String> {
    if row.menu_type == "catalog" {
        return None;
    }

    if let Some(path) = map_supported_frontend_path(row.route_path.as_deref()) {
        return Some(path.to_string());
    }

    match row.permission_code.as_deref() {
        Some("dashboard:view") => Some("/dashboard".to_string()),
        Some("system:user:list") => Some("/placeholder/users".to_string()),
        Some("system:role:list") => Some("/placeholder/roles".to_string()),
        Some("system:menu:list") => Some("/placeholder/menus".to_string()),
        Some("system:dept:list") => Some("/placeholder/departments".to_string()),
        Some("system:post:list") => Some("/placeholder/posts".to_string()),
        Some("system:dict:list") => Some("/placeholder/dicts".to_string()),
        Some("system:config:list") => Some("/system".to_string()),
        Some("system:log:login:list") => Some("/placeholder/login-logs".to_string()),
        Some("system:log:operation:list") => Some("/placeholder/audit-logs".to_string()),
        _ => row
            .route_path
            .as_deref()
            .map(to_placeholder_path)
            .or_else(|| Some(format!("/placeholder/menu-{}", row.id))),
    }
}

fn map_supported_frontend_path(route_path: Option<&str>) -> Option<&'static str> {
    match route_path {
        Some("/dashboard") | Some("/dashboard/workbench") => Some("/dashboard"),
        Some("/system") | Some("/system/config") => Some("/system"),
        Some("/profile") => Some("/profile"),
        _ => None,
    }
}

fn to_placeholder_path(route_path: &str) -> String {
    let feature = route_path
        .trim_matches('/')
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or("feature");

    format!("/placeholder/{feature}")
}

fn normalize_menu_icon(row: &MenuRow) -> Option<String> {
    let icon = row.icon.as_deref()?;

    let normalized = match icon {
        "House" | "Monitor" => "histogram",
        "User" => "user",
        "UserFilled" => "collection-tag",
        "Menu" | "CollectionTag" => "document",
        "OfficeBuilding" | "Suitcase" => "credit-card",
        "Tools" | "Setting" => "setting",
        "Tickets" | "Notebook" => "wallet",
        _ => "document",
    };

    Some(normalized.to_string())
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

#[cfg(test)]
mod tests {
    use super::{MenuRow, build_menu_tree, normalize_menu_path};

    fn menu_row(
        id: i64,
        parent_id: i64,
        menu_name: &str,
        menu_type: &str,
        route_path: Option<&str>,
        permission_code: Option<&str>,
    ) -> MenuRow {
        MenuRow {
            id,
            parent_id,
            menu_name: menu_name.to_string(),
            menu_type: menu_type.to_string(),
            route_name: Some(format!("menu-{id}")),
            route_path: route_path.map(str::to_string),
            permission_code: permission_code.map(str::to_string),
            icon: Some("Document".to_string()),
            visible: true,
            sort_no: 1,
        }
    }

    #[test]
    fn build_menu_tree_prunes_empty_catalogs() {
        let menus = build_menu_tree(vec![
            menu_row(20000, 0, "首页", "catalog", Some("/dashboard"), None),
            menu_row(
                20001,
                20000,
                "工作台",
                "menu",
                Some("/dashboard/workbench"),
                Some("dashboard:view"),
            ),
            menu_row(21000, 0, "系统管理", "catalog", Some("/system"), None),
        ]);

        assert_eq!(menus.len(), 1);
        assert_eq!(menus[0].id, "20000");
        assert_eq!(menus[0].children.len(), 1);
        assert_eq!(menus[0].children[0].path.as_deref(), Some("/dashboard"));
    }

    #[test]
    fn normalize_menu_path_uses_placeholder_for_unknown_pages() {
        let menu = menu_row(
            22020,
            22000,
            "操作日志",
            "menu",
            Some("/audit/operation-log"),
            None,
        );

        assert_eq!(
            normalize_menu_path(&menu).as_deref(),
            Some("/placeholder/operation-log")
        );
    }
}
