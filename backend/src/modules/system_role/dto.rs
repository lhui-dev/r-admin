use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionSummary {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionTreeNode {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub children: Vec<RolePermissionTreeNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RolePermissionConfigRole {
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
}

#[derive(Debug, Serialize)]
pub struct RolePermissionConfigData {
    pub role: RolePermissionConfigRole,
    pub permission_tree: Vec<RolePermissionTreeNode>,
    pub checked_permission_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolePermissionsRequest {
    pub permission_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RolePermissionMutationData {
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
