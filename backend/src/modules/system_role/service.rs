use sqlx::{FromRow, Postgres, Transaction};
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{
    common::error::{AppError, AppResult},
    modules::system_role::dto::{
        CreateRoleRequest, PaginationMeta, RoleDetailData, RoleListData, RoleListItem,
        RoleListQuery, RoleMutationData, RolePermissionConfigData, RolePermissionMutationData,
        RolePermissionSummary, RolePermissionTreeNode, RoleStatusMutationData,
        UpdateRolePermissionsRequest, UpdateRoleRequest, UpdateRoleStatusRequest,
    },
    state::AppState,
};

#[derive(Debug, Clone, FromRow)]
struct RoleListRow {
    id: i64,
    code: String,
    name: String,
    status: i16,
    data_scope: Option<String>,
    sort: i32,
    is_builtin: bool,
    user_count: i64,
    permission_count: i64,
    remark: Option<String>,
    created_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct RoleDetailRow {
    id: i64,
    code: String,
    name: String,
    status: i16,
    data_scope: Option<String>,
    sort: i32,
    is_builtin: bool,
    user_count: i64,
    permission_count: i64,
    remark: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct RoleIdentityRow {
    id: i64,
    code: String,
    name: String,
}

#[derive(Debug, Clone, FromRow)]
struct PermissionSummaryRow {
    permission_code: String,
    permission_name: String,
}

#[derive(Debug, Clone, FromRow)]
struct PermissionRow {
    id: i64,
    permission_name: String,
    permission_code: String,
    permission_type: String,
}

#[derive(Debug, Clone, FromRow)]
struct MenuRow {
    id: i64,
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    permission_code: Option<String>,
    sort_no: i32,
}

#[derive(Debug, Clone, FromRow)]
struct NextIdRow {
    next_id: i64,
}

#[derive(Debug, Clone, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, Clone, FromRow)]
struct ExistsRow {
    exists_flag: bool,
}

pub async fn list_roles(state: &AppState, query: RoleListQuery) -> AppResult<RoleListData> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let offset = ((page - 1) * page_size) as i64;
    let keyword = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{value}%"));
    let status = query.status.map(validate_status).transpose()?;
    let data_scope = query
        .data_scope
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(normalize_data_scope_filter)
        .transpose()?;

    let total_row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_role r
        WHERE r.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR r.role_name ILIKE $1
                OR r.role_code ILIKE $1
                OR COALESCE(r.remark, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR r.status = $2)
          AND (
                $3::TEXT IS NULL
                OR CASE
                    WHEN r.data_scope IN ('ALL', 'all') THEN 'all'
                    WHEN r.data_scope IN ('TENANT', 'tenant') THEN 'tenant'
                    WHEN r.data_scope IN ('DEPT', 'department', 'DEPT_AND_CHILD') THEN 'department'
                    WHEN r.data_scope IN ('CUSTOM', 'custom') THEN 'custom'
                    WHEN r.data_scope IN ('SELF', 'self') THEN 'self'
                    ELSE LOWER(COALESCE(r.data_scope, ''))
                END = $3
          )
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .bind(data_scope.as_deref())
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, RoleListRow>(
        r#"
        SELECT
            r.id,
            r.role_code AS code,
            r.role_name AS name,
            r.status,
            r.data_scope,
            r.role_sort AS sort,
            r.is_builtin,
            COALESCE(ur.user_count, 0) AS user_count,
            COALESCE(rp.permission_count, 0) AS permission_count,
            r.remark,
            TO_CHAR(r.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM sys_role r
        LEFT JOIN (
            SELECT ur.role_id, COUNT(*)::BIGINT AS user_count
            FROM sys_user_role ur
            INNER JOIN sys_user u ON u.id = ur.user_id
            WHERE u.is_deleted = FALSE
            GROUP BY ur.role_id
        ) ur ON ur.role_id = r.id
        LEFT JOIN (
            SELECT rp.role_id, COUNT(*)::BIGINT AS permission_count
            FROM sys_role_permission rp
            INNER JOIN sys_permission p ON p.id = rp.permission_id
            WHERE p.is_deleted = FALSE
              AND p.status = 1
            GROUP BY rp.role_id
        ) rp ON rp.role_id = r.id
        WHERE r.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR r.role_name ILIKE $1
                OR r.role_code ILIKE $1
                OR COALESCE(r.remark, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR r.status = $2)
          AND (
                $3::TEXT IS NULL
                OR CASE
                    WHEN r.data_scope IN ('ALL', 'all') THEN 'all'
                    WHEN r.data_scope IN ('TENANT', 'tenant') THEN 'tenant'
                    WHEN r.data_scope IN ('DEPT', 'department', 'DEPT_AND_CHILD') THEN 'department'
                    WHEN r.data_scope IN ('CUSTOM', 'custom') THEN 'custom'
                    WHEN r.data_scope IN ('SELF', 'self') THEN 'self'
                    ELSE LOWER(COALESCE(r.data_scope, ''))
                END = $3
          )
        ORDER BY r.role_sort, r.id
        OFFSET $4
        LIMIT $5
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .bind(data_scope.as_deref())
    .bind(offset)
    .bind(page_size as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(RoleListData {
        items: rows.into_iter().map(map_role_list_item).collect(),
        pagination: PaginationMeta {
            page,
            page_size,
            total: total_row.total.max(0) as u64,
        },
    })
}

pub async fn get_role_detail(state: &AppState, role_id: i64) -> AppResult<RoleDetailData> {
    let row = find_role_detail_row(state, role_id).await?;
    map_role_detail(state, row).await
}

pub async fn create_role(
    state: &AppState,
    operator_user_id: i64,
    payload: CreateRoleRequest,
) -> AppResult<RoleMutationData> {
    let code = require_non_empty(payload.code, "code")?;
    let name = require_non_empty(payload.name, "name")?;
    let status = validate_status(payload.status.unwrap_or(1))?;
    let data_scope = normalize_data_scope_input(payload.data_scope)?;
    let sort = validate_sort(payload.sort.unwrap_or(100))?;
    let remark = normalize_optional_field(payload.remark);

    let mut tx = state.db.begin().await?;
    ensure_unique_role_code(&mut tx, &code, None).await?;

    let new_role_id = next_role_id(&mut tx).await?;

    sqlx::query(
        r#"
        INSERT INTO sys_role (
            id,
            role_name,
            role_code,
            role_sort,
            data_scope,
            status,
            is_builtin,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, $5, $6, FALSE, NOW(), NOW(), $7, $7, FALSE, $8
        )
        "#,
    )
    .bind(new_role_id)
    .bind(&name)
    .bind(&code)
    .bind(sort)
    .bind(&data_scope)
    .bind(status)
    .bind(operator_user_id)
    .bind(remark)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(RoleMutationData {
        id: new_role_id,
        code,
        name,
    })
}

pub async fn update_role(
    state: &AppState,
    operator_user_id: i64,
    role_id: i64,
    payload: UpdateRoleRequest,
) -> AppResult<RoleMutationData> {
    let mut tx = state.db.begin().await?;
    let current_role = find_role_identity(&mut tx, role_id)
        .await?
        .ok_or_else(|| AppError::not_found("role not found"))?;

    let name = payload
        .name
        .map(|value| require_non_empty(value, "name"))
        .transpose()?;
    let status = payload.status.map(validate_status).transpose()?;
    let data_scope = payload
        .data_scope
        .map(normalize_data_scope_input)
        .transpose()?;
    let sort = payload.sort.map(validate_sort).transpose()?;
    let remark = payload
        .remark
        .map(|value| normalize_optional_field(Some(value)));

    if name.is_none()
        && status.is_none()
        && data_scope.is_none()
        && sort.is_none()
        && remark.is_none()
    {
        return Err(AppError::bad_request(
            "at least one updatable field is required",
        ));
    }

    if current_role.code == "super_admin" && status == Some(0) {
        return Err(AppError::forbidden("super admin role cannot be disabled"));
    }

    sqlx::query(
        r#"
        UPDATE sys_role
        SET role_name = COALESCE($2, role_name),
            status = COALESCE($3, status),
            data_scope = COALESCE($4, data_scope),
            role_sort = COALESCE($5, role_sort),
            remark = COALESCE($6, remark),
            updated_at = NOW(),
            updated_by = $7
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(role_id)
    .bind(name.as_deref())
    .bind(status)
    .bind(data_scope.as_deref())
    .bind(sort)
    .bind(remark.flatten())
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(RoleMutationData {
        id: current_role.id,
        code: current_role.code,
        name: name.unwrap_or(current_role.name),
    })
}

pub async fn update_role_status(
    state: &AppState,
    operator_user_id: i64,
    role_id: i64,
    payload: UpdateRoleStatusRequest,
) -> AppResult<RoleStatusMutationData> {
    let status = validate_status(payload.status)?;
    let mut tx = state.db.begin().await?;
    let current_role = find_role_identity(&mut tx, role_id)
        .await?
        .ok_or_else(|| AppError::not_found("role not found"))?;

    if current_role.code == "super_admin" && status == 0 {
        return Err(AppError::forbidden("super admin role cannot be disabled"));
    }

    sqlx::query(
        r#"
        UPDATE sys_role
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(role_id)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(RoleStatusMutationData {
        id: current_role.id,
        code: current_role.code,
        name: current_role.name,
        status,
    })
}

pub async fn get_role_permission_config(
    state: &AppState,
    role_id: i64,
) -> AppResult<RolePermissionConfigData> {
    let role = get_role_detail(state, role_id).await?;
    let checked_permission_ids = role
        .permissions
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let permission_tree = build_permission_tree(state).await?;

    Ok(RolePermissionConfigData {
        role,
        permission_tree,
        checked_permission_ids,
    })
}

pub async fn update_role_permissions(
    state: &AppState,
    operator_user_id: i64,
    role_id: i64,
    payload: UpdateRolePermissionsRequest,
) -> AppResult<RolePermissionMutationData> {
    let next_permission_codes = normalize_permission_codes(payload.permission_ids)?;

    let mut tx = state.db.begin().await?;
    ensure_role_exists(&mut tx, role_id).await?;

    let valid_permissions = find_permissions_by_codes(&mut tx, &next_permission_codes).await?;
    if valid_permissions.len() != next_permission_codes.len() {
        let valid_codes = BTreeSet::from_iter(
            valid_permissions
                .iter()
                .map(|permission| permission.permission_code.clone()),
        );
        let invalid_codes = next_permission_codes
            .iter()
            .filter(|code| !valid_codes.contains(*code))
            .cloned()
            .collect::<Vec<_>>();

        return Err(AppError::conflict(format!(
            "permission_id does not exist: {}",
            invalid_codes.join(", ")
        )));
    }

    sqlx::query(
        r#"
        DELETE FROM sys_role_permission
        WHERE role_id = $1
        "#,
    )
    .bind(role_id)
    .execute(&mut *tx)
    .await?;

    if !valid_permissions.is_empty() {
        let mut next_id = next_role_permission_id(&mut tx).await?;

        for permission in &valid_permissions {
            sqlx::query(
                r#"
                INSERT INTO sys_role_permission (
                    id,
                    role_id,
                    permission_id,
                    created_at,
                    created_by
                ) VALUES ($1, $2, $3, NOW(), $4)
                "#,
            )
            .bind(next_id)
            .bind(role_id)
            .bind(permission.id)
            .bind(operator_user_id)
            .execute(&mut *tx)
            .await?;

            next_id += 1;
        }
    }

    sqlx::query(
        r#"
        UPDATE sys_role
        SET updated_at = NOW(),
            updated_by = $2
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(role_id)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    get_role_detail(state, role_id).await
}

async fn find_role_detail_row(state: &AppState, role_id: i64) -> AppResult<RoleDetailRow> {
    sqlx::query_as::<_, RoleDetailRow>(
        r#"
        SELECT
            r.id,
            r.role_code AS code,
            r.role_name AS name,
            r.status,
            r.data_scope,
            r.role_sort AS sort,
            r.is_builtin,
            COALESCE((
                SELECT COUNT(*)::BIGINT
                FROM sys_user_role ur
                INNER JOIN sys_user u ON u.id = ur.user_id
                WHERE ur.role_id = r.id
                  AND u.is_deleted = FALSE
            ), 0) AS user_count,
            COALESCE((
                SELECT COUNT(*)::BIGINT
                FROM sys_role_permission rp
                INNER JOIN sys_permission p ON p.id = rp.permission_id
                WHERE rp.role_id = r.id
                  AND p.is_deleted = FALSE
                  AND p.status = 1
            ), 0) AS permission_count,
            r.remark,
            TO_CHAR(r.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(r.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_role r
        WHERE r.id = $1
          AND r.is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(role_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("role not found"))
}

async fn map_role_detail(state: &AppState, row: RoleDetailRow) -> AppResult<RoleDetailData> {
    let permissions = find_role_permission_summaries(state, row.id).await?;

    Ok(RoleDetailData {
        id: row.id,
        code: row.code,
        name: row.name,
        status: row.status,
        data_scope: normalize_data_scope_output(row.data_scope.as_deref()),
        sort: row.sort,
        is_builtin: row.is_builtin,
        user_count: row.user_count,
        permission_count: row.permission_count,
        remark: row.remark,
        created_at: row.created_at,
        updated_at: row.updated_at,
        permissions,
    })
}

fn map_role_list_item(row: RoleListRow) -> RoleListItem {
    RoleListItem {
        id: row.id,
        code: row.code,
        name: row.name,
        status: row.status,
        data_scope: normalize_data_scope_output(row.data_scope.as_deref()),
        sort: row.sort,
        is_builtin: row.is_builtin,
        user_count: row.user_count,
        permission_count: row.permission_count,
        remark: row.remark,
        created_at: row.created_at,
    }
}

async fn find_role_permission_summaries(
    state: &AppState,
    role_id: i64,
) -> AppResult<Vec<RolePermissionSummary>> {
    let rows = sqlx::query_as::<_, PermissionSummaryRow>(
        r#"
        SELECT
            p.permission_code,
            p.permission_name
        FROM sys_role_permission rp
        INNER JOIN sys_permission p ON p.id = rp.permission_id
        WHERE rp.role_id = $1
          AND p.is_deleted = FALSE
          AND p.status = 1
        ORDER BY p.permission_code
        "#,
    )
    .bind(role_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| RolePermissionSummary {
            id: row.permission_code,
            name: row.permission_name,
        })
        .collect())
}

async fn build_permission_tree(state: &AppState) -> AppResult<Vec<RolePermissionTreeNode>> {
    let (menu_rows, permission_rows) = tokio::try_join!(
        find_active_menu_rows(state),
        find_active_permission_rows(state)
    )?;

    let tree = assemble_permission_tree(menu_rows, permission_rows);
    Ok(tree)
}

async fn find_active_menu_rows(state: &AppState) -> AppResult<Vec<MenuRow>> {
    sqlx::query_as::<_, MenuRow>(
        r#"
        SELECT
            id,
            parent_id,
            menu_name,
            menu_type,
            permission_code,
            sort_no
        FROM sys_menu
        WHERE is_deleted = FALSE
          AND status = 1
          AND menu_type IN ('catalog', 'menu', 'button', 'api')
        ORDER BY parent_id, sort_no, id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

async fn find_active_permission_rows(state: &AppState) -> AppResult<Vec<PermissionRow>> {
    sqlx::query_as::<_, PermissionRow>(
        r#"
        SELECT
            id,
            permission_name,
            permission_code,
            permission_type
        FROM sys_permission
        WHERE is_deleted = FALSE
          AND status = 1
        ORDER BY permission_code
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

fn assemble_permission_tree(
    menu_rows: Vec<MenuRow>,
    permission_rows: Vec<PermissionRow>,
) -> Vec<RolePermissionTreeNode> {
    let valid_permission_codes = permission_rows
        .iter()
        .map(|permission| permission.permission_code.clone())
        .collect::<HashSet<_>>();
    let menu_leaf_permission_codes = menu_rows
        .iter()
        .filter(|row| matches!(row.menu_type.as_str(), "button" | "api"))
        .filter_map(|row| row.permission_code.as_deref())
        .map(ToOwned::to_owned)
        .collect::<HashSet<_>>();
    let menu_lookup = menu_rows
        .iter()
        .cloned()
        .map(|row| (row.id, row))
        .collect::<HashMap<_, _>>();
    let menu_permission_nodes =
        group_permissions_by_menu(&menu_rows, permission_rows, &menu_leaf_permission_codes);

    let mut children_by_parent: HashMap<i64, Vec<MenuRow>> = HashMap::new();
    for row in &menu_rows {
        children_by_parent
            .entry(row.parent_id)
            .or_default()
            .push(row.clone());
    }

    for children in children_by_parent.values_mut() {
        children.sort_by_key(|row| (row.sort_no, row.id));
    }

    let mut root_nodes = build_menu_nodes(
        0,
        &children_by_parent,
        &menu_permission_nodes,
        &menu_lookup,
        &valid_permission_codes,
    );
    if let Some(ungrouped_module) = build_ungrouped_module(&menu_permission_nodes) {
        root_nodes.push(ungrouped_module);
    }

    root_nodes
}

fn group_permissions_by_menu(
    menu_rows: &[MenuRow],
    permission_rows: Vec<PermissionRow>,
    menu_leaf_permission_codes: &HashSet<String>,
) -> HashMap<i64, Vec<RolePermissionTreeNode>> {
    let menu_prefixes = menu_rows
        .iter()
        .filter(|row| row.menu_type == "menu")
        .filter_map(|row| {
            permission_namespace(row.permission_code.as_deref()).map(|prefix| {
                let prefix_len = prefix.len();
                (row.id, prefix, prefix_len)
            })
        })
        .collect::<Vec<_>>();

    let mut permission_nodes_by_menu: HashMap<i64, Vec<RolePermissionTreeNode>> = HashMap::new();
    let mut ungrouped_permissions = Vec::new();

    for permission in permission_rows {
        if menu_leaf_permission_codes.contains(&permission.permission_code) {
            continue;
        }

        let matched_menu_id = menu_prefixes
            .iter()
            .filter(|(_, prefix, _)| permission.permission_code.starts_with(prefix))
            .max_by_key(|(_, _, prefix_len)| *prefix_len)
            .map(|(menu_id, _, _)| *menu_id);

        let node = map_permission_leaf(permission);
        if let Some(menu_id) = matched_menu_id {
            permission_nodes_by_menu
                .entry(menu_id)
                .or_default()
                .push(node);
        } else {
            ungrouped_permissions.push(node);
        }
    }

    for permissions in permission_nodes_by_menu.values_mut() {
        permissions.sort_by(|left, right| left.id.cmp(&right.id));
    }

    if !ungrouped_permissions.is_empty() {
        ungrouped_permissions.sort_by(|left, right| left.id.cmp(&right.id));
        permission_nodes_by_menu.insert(-1, ungrouped_permissions);
    }

    permission_nodes_by_menu
}

fn build_menu_nodes(
    parent_id: i64,
    children_by_parent: &HashMap<i64, Vec<MenuRow>>,
    permission_nodes_by_menu: &HashMap<i64, Vec<RolePermissionTreeNode>>,
    menu_lookup: &HashMap<i64, MenuRow>,
    valid_permission_codes: &HashSet<String>,
) -> Vec<RolePermissionTreeNode> {
    children_by_parent
        .get(&parent_id)
        .into_iter()
        .flatten()
        .filter_map(|row| {
            // Button/API menu nodes become assignable leaves only when they
            // already have a backing sys_permission record.
            if matches!(row.menu_type.as_str(), "button" | "api") {
                let permission_code = row.permission_code.as_deref()?;
                if !valid_permission_codes.contains(permission_code) {
                    return None;
                }

                return Some(RolePermissionTreeNode {
                    id: permission_code.to_string(),
                    name: row.menu_name.clone(),
                    r#type: row.menu_type.clone(),
                    children: Vec::new(),
                });
            }

            let mut children = build_menu_nodes(
                row.id,
                children_by_parent,
                permission_nodes_by_menu,
                menu_lookup,
                valid_permission_codes,
            );

            if row.menu_type == "menu" {
                if let Some(permission_nodes) = permission_nodes_by_menu.get(&row.id) {
                    children.extend(permission_nodes.clone());
                }
            }

            if row.menu_type == "catalog" && children.is_empty() {
                return None;
            }

            if row.menu_type == "menu" && children.is_empty() {
                return None;
            }

            Some(RolePermissionTreeNode {
                id: format!("{}:{}", menu_node_type(row), row.id),
                name: row.menu_name.clone(),
                r#type: menu_node_type(row).to_string(),
                children,
            })
        })
        .filter(|node| {
            if node.r#type != "menu" {
                return true;
            }

            let menu_id = node
                .id
                .strip_prefix("menu:")
                .and_then(|value| value.parse::<i64>().ok());

            if let Some(menu_id) = menu_id {
                return menu_lookup.contains_key(&menu_id);
            }

            true
        })
        .collect()
}

fn build_ungrouped_module(
    permission_nodes_by_menu: &HashMap<i64, Vec<RolePermissionTreeNode>>,
) -> Option<RolePermissionTreeNode> {
    let children = permission_nodes_by_menu.get(&-1)?.clone();

    Some(RolePermissionTreeNode {
        id: "module:ungrouped".to_string(),
        name: "未分组权限".to_string(),
        r#type: "module".to_string(),
        children: vec![RolePermissionTreeNode {
            id: "menu:ungrouped".to_string(),
            name: "其他权限".to_string(),
            r#type: "menu".to_string(),
            children,
        }],
    })
}

fn permission_namespace(permission_code: Option<&str>) -> Option<String> {
    let permission_code = permission_code?.trim();
    let (prefix, _) = permission_code.rsplit_once(':')?;
    Some(format!("{prefix}:"))
}

fn map_permission_leaf(permission: PermissionRow) -> RolePermissionTreeNode {
    RolePermissionTreeNode {
        id: permission.permission_code,
        name: permission.permission_name,
        r#type: map_permission_leaf_type(&permission.permission_type).to_string(),
        children: Vec::new(),
    }
}

fn map_permission_leaf_type(permission_type: &str) -> &'static str {
    match permission_type {
        "api" => "api",
        _ => "button",
    }
}

fn menu_node_type(row: &MenuRow) -> &'static str {
    match row.menu_type.as_str() {
        "catalog" => "module",
        _ => "menu",
    }
}

fn normalize_data_scope_output(value: Option<&str>) -> String {
    match value.unwrap_or_default() {
        "ALL" | "all" => "all".to_string(),
        "TENANT" | "tenant" => "tenant".to_string(),
        "DEPT" | "department" => "department".to_string(),
        "DEPT_AND_CHILD" => "department".to_string(),
        "CUSTOM" | "custom" => "custom".to_string(),
        "SELF" | "self" => "self".to_string(),
        other if !other.is_empty() => other.to_lowercase(),
        _ => "custom".to_string(),
    }
}

fn normalize_data_scope_filter(value: &str) -> AppResult<String> {
    match value.to_ascii_lowercase().as_str() {
        "all" => Ok("all".to_string()),
        "tenant" => Ok("tenant".to_string()),
        "department" => Ok("department".to_string()),
        "custom" => Ok("custom".to_string()),
        "self" => Ok("self".to_string()),
        _ => Err(AppError::bad_request(
            "data_scope must be one of all, tenant, department, custom, self",
        )),
    }
}

fn normalize_data_scope_input(value: String) -> AppResult<String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "all" => Ok("ALL".to_string()),
        "tenant" => Ok("TENANT".to_string()),
        "department" => Ok("DEPT_AND_CHILD".to_string()),
        "custom" => Ok("CUSTOM".to_string()),
        "self" => Ok("SELF".to_string()),
        _ => Err(AppError::bad_request(
            "data_scope must be one of all, tenant, department, custom, self",
        )),
    }
}

fn normalize_permission_codes(permission_codes: Vec<String>) -> AppResult<Vec<String>> {
    let mut normalized = Vec::new();
    let mut seen = BTreeSet::new();

    for permission_code in permission_codes {
        let trimmed = permission_code.trim();
        if trimmed.is_empty() {
            return Err(AppError::bad_request(
                "permission_ids cannot contain empty values",
            ));
        }

        if seen.insert(trimmed.to_string()) {
            normalized.push(trimmed.to_string());
        }
    }

    Ok(normalized)
}

async fn ensure_role_exists(tx: &mut Transaction<'_, Postgres>, role_id: i64) -> AppResult<()> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_role
            WHERE id = $1
              AND is_deleted = FALSE
        )
        "#,
    )
    .bind(role_id)
    .fetch_one(&mut **tx)
    .await?;

    if !exists {
        return Err(AppError::not_found("role not found"));
    }

    Ok(())
}

async fn find_permissions_by_codes(
    tx: &mut Transaction<'_, Postgres>,
    permission_codes: &[String],
) -> AppResult<Vec<PermissionRow>> {
    if permission_codes.is_empty() {
        return Ok(Vec::new());
    }

    sqlx::query_as::<_, PermissionRow>(
        r#"
        SELECT
            id,
            permission_name,
            permission_code,
            permission_type
        FROM sys_permission
        WHERE permission_code = ANY($1)
          AND is_deleted = FALSE
          AND status = 1
        ORDER BY permission_code
        "#,
    )
    .bind(permission_codes)
    .fetch_all(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn next_role_permission_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 40000) + 1 AS next_id
        FROM sys_role_permission
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn next_role_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 300) + 10 AS next_id
        FROM sys_role
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

async fn ensure_unique_role_code(
    tx: &mut Transaction<'_, Postgres>,
    code: &str,
    exclude_role_id: Option<i64>,
) -> AppResult<()> {
    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_role
            WHERE role_code = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(code)
    .bind(exclude_role_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.exists_flag {
        return Err(AppError::conflict("role code already exists"));
    }

    Ok(())
}

async fn find_role_identity(
    tx: &mut Transaction<'_, Postgres>,
    role_id: i64,
) -> AppResult<Option<RoleIdentityRow>> {
    sqlx::query_as::<_, RoleIdentityRow>(
        r#"
        SELECT
            id,
            role_code AS code,
            role_name AS name
        FROM sys_role
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(role_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

fn require_non_empty(value: String, field_name: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request(format!("{field_name} is required")));
    }

    Ok(trimmed.to_string())
}

fn normalize_optional_field(value: Option<String>) -> Option<String> {
    value.and_then(|item| {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn validate_status(status: i16) -> AppResult<i16> {
    if status != 0 && status != 1 {
        return Err(AppError::bad_request("status must be 0 or 1"));
    }

    Ok(status)
}

fn validate_sort(sort: i32) -> AppResult<i32> {
    if sort < 0 {
        return Err(AppError::bad_request(
            "sort must be greater than or equal to 0",
        ));
    }

    Ok(sort)
}
