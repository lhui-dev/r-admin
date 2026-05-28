use axum::{routing::get, Router};

use crate::{modules::health::handler, state::AppState};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(handler::health))
        .with_state(state)
}
