use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub status: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostListItem {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub status: i16,
    pub user_count: i64,
    pub remark: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PostListData {
    pub items: Vec<PostListItem>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostDetailData {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub status: i16,
    pub user_count: i64,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub code: String,
    pub name: String,
    pub sort: Option<i32>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostStatusRequest {
    pub status: i16,
}

#[derive(Debug, Serialize)]
pub struct PostMutationData {
    pub id: i64,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct PostStatusMutationData {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub status: i16,
}
