use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{
    modules::{
        auth::handler as auth_handler, health::handler as health_handler,
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
        .with_state(state)
}
