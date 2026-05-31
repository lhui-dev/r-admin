use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    modules::{auth::handler as auth_handler, health::handler as health_handler},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_handler::health))
        .route("/api/auth/login", post(auth_handler::login))
        .route("/api/auth/me", get(auth_handler::me))
        .route("/api/auth/logout", post(auth_handler::logout))
        .with_state(state)
}
