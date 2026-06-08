use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub dept_id: Option<i64>,
    pub status: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserDeptSummary {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRoleSummary {
    pub id: i64,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserPostSummary {
    pub id: i64,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub real_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub status: i16,
    pub is_super_admin: bool,
    pub dept: Option<UserDeptSummary>,
    pub roles: Vec<UserRoleSummary>,
    pub posts: Vec<UserPostSummary>,
    pub last_login_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct UserListData {
    pub items: Vec<UserListItem>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct UserDetailData {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub real_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub gender: Option<i16>,
    pub status: i16,
    pub is_super_admin: bool,
    pub remark: Option<String>,
    pub dept: Option<UserDeptSummary>,
    pub roles: Vec<UserRoleSummary>,
    pub posts: Vec<UserPostSummary>,
    pub last_login_at: Option<String>,
    pub last_login_ip: Option<String>,
    pub password_updated_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub real_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub gender: Option<i16>,
    pub dept_id: Option<i64>,
    pub status: Option<i16>,
    pub role_ids: Option<Vec<i64>>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub real_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub gender: Option<i16>,
    pub dept_id: Option<i64>,
    pub role_ids: Option<Vec<i64>>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserStatusRequest {
    pub status: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRolesRequest {
    pub role_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct UserMutationData {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct UserStatusMutationData {
    pub id: i64,
    pub username: String,
    pub status: i16,
}
