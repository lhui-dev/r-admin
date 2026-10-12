use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_post::{
        dto::{
            CreatePostRequest, PostDetailData, PostListData, PostListQuery, PostMutationData,
            PostStatusMutationData, UpdatePostRequest, UpdatePostStatusRequest,
        },
        service,
    },
    state::AppState,
};

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<PostListQuery>,
) -> AppResult<Json<ApiResponse<PostListData>>> {
    let response = service::list_posts(&state, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn detail(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(post_id): Path<i64>,
) -> AppResult<Json<ApiResponse<PostDetailData>>> {
    let response = service::get_post_detail(&state, post_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePostRequest>,
) -> AppResult<Json<ApiResponse<PostMutationData>>> {
    let response = service::create_post(&state, auth_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(post_id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>,
) -> AppResult<Json<ApiResponse<PostMutationData>>> {
    let response = service::update_post(&state, auth_user.user_id, post_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(post_id): Path<i64>,
    Json(payload): Json<UpdatePostStatusRequest>,
) -> AppResult<Json<ApiResponse<PostStatusMutationData>>> {
    let response = service::update_post_status(&state, auth_user.user_id, post_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn delete(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(post_id): Path<i64>,
) -> AppResult<Json<ApiResponse<PostMutationData>>> {
    let response = service::delete_post(&state, auth_user.user_id, post_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
