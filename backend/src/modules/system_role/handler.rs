use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_role::{
        dto::{
            CreateRoleRequest, RoleDetailData, RoleListData, RoleListQuery, RoleMutationData,
            RolePermissionConfigData, RolePermissionMutationData, RoleStatusMutationData,
            UpdateRolePermissionsRequest, UpdateRoleRequest, UpdateRoleStatusRequest,
        },
        service,
    },
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<RoleListQuery>,
) -> AppResult<Json<ApiResponse<RoleListData>>> {
    let response = service::list_roles(&state, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn detail(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<i64>,
) -> AppResult<Json<ApiResponse<RoleDetailData>>> {
    let response = service::get_role_detail(&state, role_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateRoleRequest>,
) -> AppResult<Json<ApiResponse<RoleMutationData>>> {
    let response = service::create_role(&state, auth_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(role_id): Path<i64>,
    Json(payload): Json<UpdateRoleRequest>,
) -> AppResult<Json<ApiResponse<RoleMutationData>>> {
    let response = service::update_role(&state, auth_user.user_id, role_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(role_id): Path<i64>,
    Json(payload): Json<UpdateRoleStatusRequest>,
) -> AppResult<Json<ApiResponse<RoleStatusMutationData>>> {
    let response = service::update_role_status(&state, auth_user.user_id, role_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

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
    let response =
        service::update_role_permissions(&state, auth_user.user_id, role_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
