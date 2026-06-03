use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RoleListQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub status: Option<i16>,
    pub data_scope: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleListItem {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub status: i16,
    pub data_scope: String,
    pub sort: i32,
    pub is_builtin: bool,
    pub user_count: i64,
    pub permission_count: i64,
    pub remark: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct RoleListData {
    pub items: Vec<RoleListItem>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionSummary {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleDetailData {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub status: i16,
    pub data_scope: String,
    pub sort: i32,
    pub is_builtin: bool,
    pub user_count: i64,
    pub permission_count: i64,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Vec<RolePermissionSummary>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub code: String,
    pub name: String,
    pub status: Option<i16>,
    pub data_scope: String,
    pub sort: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub status: Option<i16>,
    pub data_scope: Option<String>,
    pub sort: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleStatusRequest {
    pub status: i16,
}

#[derive(Debug, Serialize)]
pub struct RoleMutationData {
    pub id: i64,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RoleStatusMutationData {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub status: i16,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionTreeNode {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub children: Vec<RolePermissionTreeNode>,
}

#[derive(Debug, Serialize)]
pub struct RolePermissionConfigData {
    pub role: RoleDetailData,
    pub permission_tree: Vec<RolePermissionTreeNode>,
    pub checked_permission_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolePermissionsRequest {
    pub permission_ids: Vec<String>,
}

pub type RolePermissionMutationData = RoleDetailData;
