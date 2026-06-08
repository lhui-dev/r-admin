use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_user::{
        dto::{
            CreateUserRequest, UpdateUserRequest, UpdateUserRolesRequest, UpdateUserStatusRequest,
            UserDetailData, UserListData, UserListQuery, UserMutationData, UserStatusMutationData,
        },
        service,
    },
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<UserListQuery>,
) -> AppResult<Json<ApiResponse<UserListData>>> {
    let response = service::list_users(&state, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn detail(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(user_id): Path<i64>,
) -> AppResult<Json<ApiResponse<UserDetailData>>> {
    let response = service::get_user_detail(&state, user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<Json<ApiResponse<UserMutationData>>> {
    let response = service::create_user(&state, auth_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserMutationData>>> {
    let response = service::update_user(&state, auth_user.user_id, user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> AppResult<Json<ApiResponse<UserStatusMutationData>>> {
    let response = service::update_user_status(&state, auth_user.user_id, user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_roles(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserRolesRequest>,
) -> AppResult<Json<ApiResponse<UserMutationData>>> {
    let response = service::update_user_roles(&state, auth_user.user_id, user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
