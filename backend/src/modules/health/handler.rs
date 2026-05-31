use axum::{Json, extract::State};
use serde::Serialize;
use sqlx::query_scalar;

use crate::{common::response::ApiResponse, state::AppState};

#[derive(Debug, Serialize)]
pub struct HealthPayload {
    pub status: &'static str,
    pub service: String,
    pub database: &'static str,
}

pub async fn health(State(state): State<AppState>) -> Json<ApiResponse<HealthPayload>> {
    let database = if query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok()
    {
        "up"
    } else {
        "down"
    };

    Json(ApiResponse::ok(HealthPayload {
        status: "ok",
        service: state.app_name().to_string(),
        database,
    }))
}
