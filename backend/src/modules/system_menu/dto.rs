use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MenuTreeQuery {
    pub keyword: Option<String>,
    pub status: Option<i16>,
    pub menu_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuTreeItem {
    pub id: i64,
    pub parent_id: i64,
    pub menu_name: String,
    pub menu_type: String,
    pub route_name: Option<String>,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub permission_code: Option<String>,
    pub icon: Option<String>,
    pub sort_no: i32,
    pub visible: bool,
    pub keep_alive: bool,
    pub is_external: bool,
    pub status: i16,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub children: Vec<MenuTreeItem>,
}

#[derive(Debug, Serialize)]
pub struct MenuTreeData {
    pub items: Vec<MenuTreeItem>,
}

pub type MenuDetailData = MenuTreeItem;

#[derive(Debug, Deserialize)]
pub struct CreateMenuRequest {
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub menu_type: String,
    pub route_name: Option<String>,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub permission_code: Option<String>,
    pub icon: Option<String>,
    pub sort_no: Option<i32>,
    pub visible: Option<bool>,
    pub keep_alive: Option<bool>,
    pub is_external: Option<bool>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuRequest {
    pub parent_id: Option<i64>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub route_name: Option<String>,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub permission_code: Option<String>,
    pub icon: Option<String>,
    pub sort_no: Option<i32>,
    pub visible: Option<bool>,
    pub keep_alive: Option<bool>,
    pub is_external: Option<bool>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuStatusRequest {
    pub status: i16,
}

#[derive(Debug, Serialize)]
pub struct MenuMutationData {
    pub id: i64,
    pub menu_name: String,
    pub menu_type: String,
}

#[derive(Debug, Serialize)]
pub struct MenuStatusMutationData {
    pub id: i64,
    pub menu_name: String,
    pub status: i16,
}
