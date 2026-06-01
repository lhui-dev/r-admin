# 后端 `/api/auth/menus` 实现准备

## 1. 目标

本文档用于承接当前动态菜单阶段的后端准备工作，在真正开始写 `/api/auth/menus` 代码之前，先把下面三件事整理清楚：

1. backend 侧 DTO 应该长什么样
2. handler 应该如何落位
3. service 查询思路应该如何拆分

当前目标不是立即把完整菜单权限系统写完，而是先把“实现路径”定清楚，避免后续直接改代码时边做边返工。

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

因此，后端现在并不是“从零开始做菜单”，而是已经具备实现 `/api/auth/menus` 的基础数据条件。

## 3. 推荐实现范围

第一版后端实现建议只做：

1. 新增 `GET /api/auth/menus`
2. 基于当前登录用户查询可见菜单
3. 返回前端可直接渲染的菜单树
4. 超级管理员先支持完整菜单返回
5. 普通角色按角色权限过滤可见菜单

第一版暂不建议做：

1. 动态路由注册元信息全量下发
2. 外链菜单
3. 多租户菜单隔离
4. 菜单缓存刷新策略
5. 三级以上复杂菜单联动

一句话原则：

`先把“当前用户能看到什么菜单”稳定返回出来，再考虑更复杂的菜单生态。`

## 4. DTO 设计建议

### 4.1 新增响应 DTO

建议在 `backend/src/modules/auth/dto.rs` 中新增：

```rust
#[derive(Debug, Serialize)]
pub struct CurrentMenuItem {
    pub id: i64,
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

### 4.2 字段说明

建议字段与前端菜单契约保持一致：

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

### 4.3 为什么使用树形 DTO

不建议第一版直接返回平铺数组让前端自己组树。

原因：

1. 菜单树属于后端权限过滤后的最终导航结构
2. 后端更清楚哪些父节点应该被保留
3. 前端只负责展示，逻辑更简单

一句话理解：

`后端负责返回“最终可见菜单树”，前端负责渲染，不负责猜树。`

## 5. Handler 落位建议

### 5.1 当前 handler 结构

当前 `handler.rs` 已有：

1. `login`
2. `me`
3. `logout`

因此菜单接口最自然的落位也是放在认证模块下。

### 5.2 建议新增 handler

建议新增：

```rust
pub async fn menus(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<CurrentMenusResponse>>> {
    let response = service::current_menus(&state, auth_user.user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
```

### 5.3 路由接入建议

在 `backend/src/routes/mod.rs` 中新增：

```rust
.route("/api/auth/menus", get(auth_handler::menus))
```

这样可以保证菜单初始化仍然沿用认证域接口，不会打散当前模块边界。

## 6. Service 设计建议

### 6.1 推荐新增 service 入口

建议在 `backend/src/modules/auth/service.rs` 中新增：

```rust
pub async fn current_menus(state: &AppState, user_id: i64) -> AppResult<CurrentMenusResponse>
```

### 6.2 推荐拆分的内部步骤

建议 service 至少拆成下面几步：

1. 先查询并校验当前用户
2. 查询用户角色
3. 查询该用户可见菜单的原始记录
4. 将原始记录组装成树
5. 返回 `CurrentMenusResponse`

这样后续如果要扩租户、缓存或菜单状态过滤，都有明确插入点。

## 7. 查询思路建议

### 7.1 超级管理员

建议策略：

1. 如果 `is_super_admin = true`
2. 直接查询全部启用、未删除菜单
3. 按 `parent_id + sort_no` 构建树

这样第一版最容易先跑通。

### 7.2 普通角色

建议策略：

1. 先查用户角色
2. 基于角色可见权限标识，过滤 `sys_menu.permission_code`
3. 保留有可见子节点的父级目录

当前库表里没有单独的 `sys_role_menu` 关系，但已有：

1. `sys_user_role`
2. `sys_role`
3. `sys_role_permission`
4. `sys_permission`
5. `sys_menu.permission_code`

因此第一版可以先走：

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

## 8. 推荐内部 Row 结构

建议在 `service.rs` 内部新增一个 `MenuRow`：

```rust
#[derive(Debug, FromRow)]
struct MenuRow {
    id: i64,
    parent_id: Option<i64>,
    menu_name: String,
    route_name: Option<String>,
    route_path: Option<String>,
    icon: Option<String>,
    permission_code: Option<String>,
    hidden: bool,
    sort_no: i32,
}
```

说明：

1. 这是数据库行结构
2. 不建议直接把数据库行结构暴露成 API DTO
3. 组树前后分成两层结构更利于维护

## 9. 组树思路建议

建议流程：

1. 先查出菜单平铺列表
2. 按 `sort_no` 排好
3. 建立 `id -> node` 的映射
4. 根据 `parent_id` 归并到父节点
5. 最终返回顶层菜单数组

第一版如果希望降低实现复杂度，也可以先：

1. 只支持两级菜单
2. 目录级菜单作为顶层
3. 可点击页面作为子节点

这和当前前端动态菜单初始化阶段是匹配的。

## 10. 推荐 SQL 思路

### 10.1 超级管理员

建议查询方向：

```sql
SELECT
    id,
    parent_id,
    menu_name,
    route_name,
    route_path,
    icon,
    permission_code,
    hidden,
    sort_no
FROM sys_menu
WHERE is_deleted = FALSE
  AND status = 1
ORDER BY parent_id NULLS FIRST, sort_no, id;
```

### 10.2 普通角色

建议查询方向：

```sql
SELECT DISTINCT
    m.id,
    m.parent_id,
    m.menu_name,
    m.route_name,
    m.route_path,
    m.icon,
    m.permission_code,
    m.hidden,
    m.sort_no
FROM sys_menu m
INNER JOIN sys_role_permission rp
    ON rp.permission_id = p.id
INNER JOIN sys_permission p
    ON p.permission_code = m.permission_code
INNER JOIN sys_user_role ur
    ON ur.role_id = rp.role_id
INNER JOIN sys_role r
    ON r.id = ur.role_id
WHERE ur.user_id = $1
  AND m.is_deleted = FALSE
  AND m.status = 1
  AND r.is_deleted = FALSE
  AND r.status = 1
  AND p.is_deleted = FALSE
  AND p.status = 1;
```

注意：

上面只是查询方向示意，真正实现时需要再补：

1. 父级目录保留策略
2. `menu_type` 过滤
3. 无权限但有可见子级的目录保留逻辑

## 11. 第一版实现顺序建议

建议真正写代码时按下面顺序推进：

1. 在 `dto.rs` 中新增菜单响应 DTO
2. 在 `handler.rs` 中新增 `menus`
3. 在 `routes/mod.rs` 注册 `/api/auth/menus`
4. 在 `service.rs` 先做超级管理员完整返回
5. 再补普通用户按权限过滤
6. 最后联调前端 fallback 切换

这样可以先快速把 `404` 消掉，再逐步提升菜单过滤准确度。

## 12. 第一版完成标准

当下面条件满足时，可认为后端 `/api/auth/menus` 准备阶段已经可以进入正式编码：

1. DTO 结构已明确
2. handler 落位已明确
3. service 入口已明确
4. 查询路径和组树方式已明确
5. 已知道第一版先做超级管理员，再补普通角色过滤

一句话总结：

`当前阶段最重要的不是立刻把菜单接口写完，而是先把“返回什么、从哪查、怎么组树、落在哪层”这四件事定稳。`
