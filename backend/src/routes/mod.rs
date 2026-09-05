use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{
    modules::{
        auth::handler as auth_handler, health::handler as health_handler,
        system_dept::handler as system_dept_handler, system_menu::handler as system_menu_handler,
        system_role::handler as system_role_handler, system_user::handler as system_user_handler,
    },
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_handler::health))
        .route("/api/auth/login", post(auth_handler::login))
        .route("/api/auth/me", get(auth_handler::me))
        .route("/api/auth/menus", get(auth_handler::menus))
        .route("/api/auth/logout", post(auth_handler::logout))
        .route(
            "/api/system/users",
            get(system_user_handler::list).post(system_user_handler::create),
        )
        .route(
            "/api/system/users/{id}",
            get(system_user_handler::detail).patch(system_user_handler::update),
        )
        .route(
            "/api/system/users/{id}/status",
            patch(system_user_handler::update_status),
        )
        .route(
            "/api/system/users/{id}/roles",
            patch(system_user_handler::update_roles),
        )
        .route(
            "/api/system/roles",
            get(system_role_handler::list).post(system_role_handler::create),
        )
        .route(
            "/api/system/roles/{id}",
            get(system_role_handler::detail).patch(system_role_handler::update),
        )
        .route(
            "/api/system/roles/{id}/status",
            patch(system_role_handler::update_status),
        )
        .route(
            "/api/system/roles/{id}/permission-config",
            get(system_role_handler::permission_config),
        )
        .route(
            "/api/system/roles/{id}/permissions",
            put(system_role_handler::update_permissions),
        )
        .route("/api/system/menus/tree", get(system_menu_handler::tree))
        .route("/api/system/menus", post(system_menu_handler::create))
        .route(
            "/api/system/menus/{id}",
            get(system_menu_handler::detail)
                .patch(system_menu_handler::update)
                .delete(system_menu_handler::delete),
        )
        .route(
            "/api/system/menus/{id}/status",
            patch(system_menu_handler::update_status),
        )
        .route("/api/system/depts/tree", get(system_dept_handler::tree))
        .route("/api/system/depts", post(system_dept_handler::create))
        .route(
            "/api/system/depts/{id}",
            get(system_dept_handler::detail)
                .patch(system_dept_handler::update)
                .delete(system_dept_handler::delete),
        )
        .route(
            "/api/system/depts/{id}/status",
            patch(system_dept_handler::update_status),
        )
        .with_state(state)
}
