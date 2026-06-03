use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_role::{
        dto::{RolePermissionConfigData, RolePermissionMutationData, UpdateRolePermissionsRequest},
        service,
    },
    state::AppState,
};

pub async fn permission_config(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<i64>,
) -> AppResult<Json<ApiResponse<RolePermissionConfigData>>> {
    let response = service::get_role_permission_config(&state, role_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(role_id): Path<i64>,
    Json(payload): Json<UpdateRolePermissionsRequest>,
) -> AppResult<Json<ApiResponse<RolePermissionMutationData>>> {
    let response = service::update_role_permissions(&state, auth_user.user_id, role_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
