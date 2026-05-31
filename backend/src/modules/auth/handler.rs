use axum::{Json, extract::State};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::auth::{
        dto::{LoginRequest, LogoutResponse},
        service,
    },
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<crate::modules::auth::dto::LoginResponse>>> {
    let response = service::login(&state, &payload.username, &payload.password).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<crate::modules::auth::dto::CurrentUserResponse>>> {
    let response = service::current_user(&state, auth_user.user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<LogoutResponse>>> {
    state
        .revoke_token(auth_user.access_token, auth_user.access_token_expires_at)
        .await;

    Ok(Json(ApiResponse::ok(LogoutResponse { logged_out: true })))
}
