use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    common::{error::AppResult, response::ApiResponse},
    middleware::auth::AuthUser,
    modules::system_dept::{
        dto::{
            CreateDeptRequest, DeptDetailData, DeptMutationData, DeptStatusMutationData,
            DeptTreeData, DeptTreeQuery, UpdateDeptRequest, UpdateDeptStatusRequest,
        },
        service,
    },
    state::AppState,
};

pub async fn tree(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<DeptTreeQuery>,
) -> AppResult<Json<ApiResponse<DeptTreeData>>> {
    let response = service::list_dept_tree(&state, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn detail(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(dept_id): Path<i64>,
) -> AppResult<Json<ApiResponse<DeptDetailData>>> {
    let response = service::get_dept_detail(&state, dept_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateDeptRequest>,
) -> AppResult<Json<ApiResponse<DeptMutationData>>> {
    let response = service::create_dept(&state, auth_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(dept_id): Path<i64>,
    Json(payload): Json<UpdateDeptRequest>,
) -> AppResult<Json<ApiResponse<DeptMutationData>>> {
    let response = service::update_dept(&state, auth_user.user_id, dept_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(dept_id): Path<i64>,
    Json(payload): Json<UpdateDeptStatusRequest>,
) -> AppResult<Json<ApiResponse<DeptStatusMutationData>>> {
    let response = service::update_dept_status(&state, auth_user.user_id, dept_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn delete(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(dept_id): Path<i64>,
) -> AppResult<Json<ApiResponse<DeptMutationData>>> {
    let response = service::delete_dept(&state, auth_user.user_id, dept_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
