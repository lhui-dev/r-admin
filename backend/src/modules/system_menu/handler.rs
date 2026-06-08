use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_menu::{
        dto::{
            CreateMenuRequest, MenuDetailData, MenuMutationData, MenuStatusMutationData,
            MenuTreeData, MenuTreeQuery, UpdateMenuRequest, UpdateMenuStatusRequest,
        },
        service,
    },
    state::AppState,
};

pub async fn tree(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<MenuTreeQuery>,
) -> AppResult<Json<ApiResponse<MenuTreeData>>> {
    let response = service::list_menu_tree(&state, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn detail(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<i64>,
) -> AppResult<Json<ApiResponse<MenuDetailData>>> {
    let response = service::get_menu_detail(&state, menu_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateMenuRequest>,
) -> AppResult<Json<ApiResponse<MenuMutationData>>> {
    let response = service::create_menu(&state, auth_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(menu_id): Path<i64>,
    Json(payload): Json<UpdateMenuRequest>,
) -> AppResult<Json<ApiResponse<MenuMutationData>>> {
    let response = service::update_menu(&state, auth_user.user_id, menu_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(menu_id): Path<i64>,
    Json(payload): Json<UpdateMenuStatusRequest>,
) -> AppResult<Json<ApiResponse<MenuStatusMutationData>>> {
    let response = service::update_menu_status(&state, auth_user.user_id, menu_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn delete(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(menu_id): Path<i64>,
) -> AppResult<Json<ApiResponse<MenuMutationData>>> {
    let response = service::delete_menu(&state, auth_user.user_id, menu_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
