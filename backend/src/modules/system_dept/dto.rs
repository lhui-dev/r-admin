use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeptTreeQuery {
    pub keyword: Option<String>,
    pub status: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeptTreeItem {
    pub id: i64,
    pub parent_id: i64,
    pub dept_name: String,
    pub dept_code: Option<String>,
    pub leader_user_id: Option<i64>,
    pub leader_name: Option<String>,
    pub sort_no: i32,
    pub status: i16,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub children: Vec<DeptTreeItem>,
}

#[derive(Debug, Serialize)]
pub struct DeptTreeData {
    pub items: Vec<DeptTreeItem>,
}

pub type DeptDetailData = DeptTreeItem;

#[derive(Debug, Deserialize)]
pub struct CreateDeptRequest {
    pub parent_id: Option<i64>,
    pub dept_name: String,
    pub dept_code: Option<String>,
    pub leader_user_id: Option<i64>,
    pub sort_no: Option<i32>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeptRequest {
    pub parent_id: Option<i64>,
    pub dept_name: Option<String>,
    #[serde(default)]
    pub dept_code: PatchField<String>,
    #[serde(default)]
    pub leader_user_id: PatchField<i64>,
    pub sort_no: Option<i32>,
    pub status: Option<i16>,
    #[serde(default)]
    pub remark: PatchField<String>,
}

impl UpdateDeptRequest {
    pub fn has_any_field(&self) -> bool {
        self.parent_id.is_some()
            || self.dept_name.is_some()
            || !matches!(self.dept_code, PatchField::Missing)
            || !matches!(self.leader_user_id, PatchField::Missing)
            || self.sort_no.is_some()
            || self.status.is_some()
            || !matches!(self.remark, PatchField::Missing)
    }
}

#[derive(Debug, Clone, Default)]
pub enum PatchField<T> {
    #[default]
    Missing,
    Null,
    Value(T),
}

impl<'de, T> Deserialize<'de> for PatchField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => PatchField::Value(value),
            None => PatchField::Null,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeptStatusRequest {
    pub status: i16,
}

#[derive(Debug, Serialize)]
pub struct DeptMutationData {
    pub id: i64,
    pub dept_name: String,
}

#[derive(Debug, Serialize)]
pub struct DeptStatusMutationData {
    pub id: i64,
    pub dept_name: String,
    pub status: i16,
}
