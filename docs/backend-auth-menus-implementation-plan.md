# 后端 `/api/auth/menus` 实现说明

## 1. 目标

本文档最初用于承接动态菜单阶段的后端设计准备；截至 2026-06-01，`GET /api/auth/menus` 第一版已经落地，因此当前文档改为说明：

1. 已经实现了什么
2. 实际实现与最初设计有什么差异
3. 后续应如何继续推进

## 2. 当前基础

当前后端认证模块已经具备：

1. `POST /api/auth/login`
2. `GET /api/auth/me`
3. `POST /api/auth/logout`

现有代码位置：

1. [dto.rs](../backend/src/modules/auth/dto.rs)
2. [handler.rs](../backend/src/modules/auth/handler.rs)
3. [service.rs](../backend/src/modules/auth/service.rs)
4. [routes/mod.rs](../backend/src/routes/mod.rs)

当前菜单相关数据基础已经存在：

1. `sys_menu`
2. `sys_user_role`
3. `sys_role`
4. 权限标识字段 `permission_code`

因此，`/api/auth/menus` 已经不再是“待设计接口”，而是已具备真实联调能力。

## 3. 当前已实现范围

当前第一版已经实现：

1. 新增 `GET /api/auth/menus`
2. 基于当前登录用户查询可见菜单
3. 返回前端可直接渲染的菜单树
4. 超级管理员返回完整可见菜单
5. 普通角色按角色权限过滤可见菜单
6. 空目录菜单不返回
7. 当前前端已存在页面自动映射到真实路由
8. 未落地页面统一映射到 `placeholder` 路由

当前仍未纳入：

1. 动态路由注册元信息全量下发
2. 外链菜单
3. 多租户菜单隔离
4. 菜单缓存刷新策略
5. 三级以上复杂菜单联动

一句话原则：

`先把“当前用户能看到什么菜单”稳定返回出来，再考虑更复杂的菜单生态。`

## 4. 当前 DTO 与响应结构

当前已落地 DTO：

```rust
#[derive(Debug, Serialize)]
pub struct CurrentMenuItem {
    pub id: String,
    pub name: String,
    pub title: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub permission: Option<String>,
    pub hidden: bool,
    pub children: Vec<CurrentMenuItem>,
}

#[derive(Debug, Serialize)]
pub struct CurrentMenusResponse {
    pub menus: Vec<CurrentMenuItem>,
}
```

与最初设计相比，当前 `id` 使用 `String`，这是为了和前端菜单类型保持更稳定的契约。

### 4.1 字段说明

当前字段与前端菜单契约保持一致：

1. `id`
   直接对应菜单表主键，方便稳定引用
2. `name`
   推荐对应 `route_name` 或后端整理后的业务标识
3. `title`
   推荐对应 `menu_name`
4. `path`
   推荐对应 `route_path`
5. `icon`
   推荐对应菜单表的图标字段
6. `permission`
   推荐对应 `permission_code`
7. `hidden`
   对应菜单是否隐藏
8. `children`
   返回子菜单数组，保持前端结构稳定

### 4.2 为什么仍然使用树形 DTO

不建议第一版直接返回平铺数组让前端自己组树。

原因：

1. 菜单树属于后端权限过滤后的最终导航结构
2. 后端更清楚哪些父节点应该被保留
3. 前端只负责展示，逻辑更简单

一句话理解：

`后端负责返回“最终可见菜单树”，前端负责渲染，不负责猜树。`

## 5. Handler 与路由落位

当前实现已落位到认证模块下，handler 结构如下：

```rust
pub async fn menus(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<CurrentMenusResponse>>> {
    let response = service::current_menus(&state, auth_user.user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
```

当前路由已注册：

```rust
.route("/api/auth/menus", get(auth_handler::menus))
```

## 6. Service 实现结构

当前 service 主入口已实现：

```rust
pub async fn current_menus(state: &AppState, user_id: i64) -> AppResult<CurrentMenusResponse>
```

当前实现实际拆成了以下几个步骤：

1. 先查询并校验当前用户
2. 超级管理员走全量菜单查询
3. 普通角色走基于权限标识的菜单过滤查询
4. 将菜单原始记录组装成树
5. 过滤空目录
6. 归一化路径与图标
7. 返回 `CurrentMenusResponse`

## 7. 查询思路与当前行为

### 7.1 超级管理员

当前策略：

1. 如果 `is_super_admin = true`
2. 直接查询全部启用、未删除菜单
3. 按 `parent_id + sort_no` 构建树

### 7.2 普通角色

当前策略：

1. 先查用户角色
2. 基于角色可见权限标识，过滤 `sys_menu.permission_code`
3. 保留有可见子节点的父级目录

当前库表里没有单独的 `sys_role_menu` 关系，但已有：

1. `sys_user_role`
2. `sys_role`
3. `sys_role_permission`
4. `sys_permission`
5. `sys_menu.permission_code`

当前第一版实际采用：

`用户 -> 角色 -> 权限标识 -> 用 permission_code 关联 sys_menu`

这与已有 RBAC 设计是一致的。

### 7.3 推荐原始查询字段

建议先查出下面这些字段：

1. `id`
2. `parent_id`
3. `menu_name`
4. `route_name`
5. `route_path`
6. `icon`
7. `permission_code`
8. `hidden`
9. `sort_no`
10. `status`
11. `is_deleted`

这样后续组树时信息足够，不用重复回表。

## 8. 当前内部 Row 结构

当前 `service.rs` 中使用的数据库行结构为：

```rust
#[derive(Debug, Clone, FromRow)]
struct MenuRow {
    id: i64,
    parent_id: i64,
    menu_name: String,
    menu_type: String,
    route_name: Option<String>,
    route_path: Option<String>,
    icon: Option<String>,
    permission_code: Option<String>,
    visible: bool,
    sort_no: i32,
}
```

## 9. 组树与归一化逻辑

当前流程：

1. 先查出菜单平铺列表
2. 按 `parent_id + sort_no + id` 排好
3. 基于 `parent_id` 构建树
4. 目录节点若无可见子节点则过滤掉
5. 已落地页面映射到真实路由
6. 未落地页面映射到 `placeholder`
7. 图标名称归一化为前端可识别字符串

## 10. 当前联调结果

已完成真实接口验证：

1. `admin` 返回 `首页`、`系统管理`、`日志审计`
2. `sysadmin` 返回 `首页`、`系统管理`、`日志审计`
3. `auditor` 只返回 `首页`、`日志审计`

这说明：

1. 接口可用
2. 角色差异可用
3. 空目录过滤可用

## 11. 后续继续推进的建议

当前不建议继续停留在 `/api/auth/menus` 设计层，而应转向 RBAC 业务接口建设。

推荐下一步：

1. 用户管理接口设计与占位实现
2. 角色管理接口设计与占位实现
3. 菜单管理接口设计与占位实现
4. 为后续统一分页与列表筛选 DTO 打基础

一句话总结：

`/api/auth/menus` 第一版已经落地完成，下一步应从“菜单能不能返回”转向“系统管理业务接口如何系统化展开”。`
