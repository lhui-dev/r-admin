# 菜单管理模块实现计划

本文档用于规划 `r-admin` 菜单管理模块的后续实现。当前系统已经具备 `/api/auth/menus` 当前用户可见菜单读取能力，但还缺少面向管理员的菜单维护能力。本阶段目标是补齐 `sys_menu` 的管理闭环，并让菜单管理与角色权限配置、动态路由渲染形成稳定联动。

## 1. 当前现状

已具备能力：

1. `sys_menu` 表结构和种子菜单数据已存在。
2. `/api/auth/menus` 可以按当前用户角色返回可见菜单树。
3. 前端侧边栏已从菜单 store 渲染动态菜单。
4. 角色权限配置页已能读取菜单和权限点组成的授权树。
5. `/system/menu` 路由已存在，但当前仍落到建设中占位页。

尚未具备能力：

1. 菜单管理后端 CRUD 接口。
2. 菜单树查询接口。
3. 菜单新增、编辑、启停、删除。
4. 菜单排序、图标、路由、权限标识维护。
5. 前端菜单管理页面。
6. APIFox 菜单管理接口文档。

## 2. 本阶段目标

本阶段建议以 `feature/system-menu-management` 分支推进。

核心目标：

1. 新增后端 `system_menu` 模块。
2. 实现菜单树查询、新增、更新、启停、删除接口。
3. 前端新增 `/system/menu` 菜单管理页，替换建设中页面。
4. 菜单管理页复用已有 `base-ui` 组件风格。
5. 菜单变更后能够影响 `/api/auth/menus` 和角色权限配置的数据源。
6. 补充 APIFox/OpenAPI 文档，方便后续调试。

## 3. 后端接口建议

### 3.1 查询菜单树

```http
GET /api/system/menus/tree
```

查询参数：

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `keyword` | `string` | 否 | 菜单名称、路由路径、权限标识模糊查询 |
| `status` | `number` | 否 | 菜单状态，`1` 启用，`0` 禁用 |
| `menu_type` | `string` | 否 | `catalog`、`menu`、`button`、`api` |

返回结构：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "items": [
      {
        "id": 21000,
        "parent_id": 0,
        "menu_name": "系统管理",
        "menu_type": "catalog",
        "route_name": "System",
        "route_path": "/system",
        "component_path": null,
        "permission_code": null,
        "icon": "Setting",
        "sort_no": 20,
        "visible": true,
        "keep_alive": false,
        "is_external": false,
        "status": 1,
        "remark": "系统管理目录",
        "children": []
      }
    ]
  }
}
```

### 3.2 查询菜单详情

```http
GET /api/system/menus/{id}
```

用途：

1. 编辑弹窗回填。
2. 后续菜单详情抽屉扩展。

### 3.3 新增菜单

```http
POST /api/system/menus
```

请求示例：

```json
{
  "parent_id": 21000,
  "menu_name": "菜单管理",
  "menu_type": "menu",
  "route_name": "SystemMenu",
  "route_path": "/system/menu",
  "component_path": "system/menu/index",
  "permission_code": "system:menu:list",
  "icon": "Menu",
  "sort_no": 30,
  "visible": true,
  "keep_alive": true,
  "is_external": false,
  "status": 1,
  "remark": "菜单管理页面"
}
```

### 3.4 更新菜单

```http
PATCH /api/system/menus/{id}
```

规则：

1. 支持局部更新。
2. 不允许将菜单父级设置为自己或自己的子孙节点。
3. 不允许修改已删除菜单。
4. 目录类菜单可以没有 `route_path` 和 `component_path`。
5. 页面菜单建议必须有 `route_path`。
6. 按钮/API 权限点建议必须有 `permission_code`。

### 3.5 启停菜单

```http
PATCH /api/system/menus/{id}/status
```

请求：

```json
{
  "status": 0
}
```

规则：

1. 状态仅允许 `0` 或 `1`。
2. 禁用目录时，前端 `/api/auth/menus` 不再返回该目录及其子节点。
3. 是否级联禁用子节点建议暂不自动处理，仅通过查询时父级裁剪体现。

### 3.6 删除菜单

```http
DELETE /api/system/menus/{id}
```

规则：

1. 使用软删除。
2. 存在未删除子菜单时不允许删除。
3. 菜单已被角色权限引用时，第一版建议不允许删除。
4. 后续可以提供“删除并清理授权关系”的显式接口。

## 4. DTO 设计建议

建议新增后端模块：

```text
backend/src/modules/system_menu/
```

文件：

1. `mod.rs`
2. `dto.rs`
3. `handler.rs`
4. `service.rs`

核心 DTO：

```text
MenuTreeQuery
MenuTreeData
MenuTreeItem
MenuDetailData
CreateMenuRequest
UpdateMenuRequest
UpdateMenuStatusRequest
MenuMutationData
```

字段命名保持数据库风格，方便前后端调试：

```text
parent_id
menu_name
menu_type
route_name
route_path
component_path
permission_code
sort_no
keep_alive
is_external
```

## 5. 后端校验规则

菜单类型规则：

| 类型 | 说明 | 关键字段建议 |
| --- | --- | --- |
| `catalog` | 目录 | `menu_name`、`sort_no`、`icon` |
| `menu` | 页面菜单 | `menu_name`、`route_path`、`component_path`、`permission_code` |
| `button` | 页面按钮权限 | `menu_name`、`permission_code` |
| `api` | 接口权限点 | `menu_name`、`permission_code` |

基础校验：

1. `menu_name` 必填。
2. `menu_type` 必须在允许范围内。
3. `parent_id` 默认为 `0`。
4. `sort_no` 默认为 `0`。
5. `status` 只能是 `0` 或 `1`。
6. `permission_code` 如果填写，应保持唯一。
7. `route_path` 如果填写，应以 `/` 开头。
8. `parent_id` 指向的父菜单必须存在且未删除。
9. 更新时不允许形成循环父子关系。

## 6. 与现有模块联动

### 6.1 与 `/api/auth/menus`

菜单管理修改 `sys_menu` 后，`/api/auth/menus` 应自然受到影响。

联动点：

1. 新增启用菜单后，具备对应权限的用户可见。
2. 禁用菜单后，所有用户不可见。
3. 隐藏菜单 `visible = false` 时，动态菜单不展示。
4. 修改 `route_path` 后，前端菜单跳转路径变化。

### 6.2 与角色权限配置

角色权限配置当前依赖 `sys_menu` 和 `sys_permission` 生成授权树。

菜单管理落地后需要确认：

1. 新增菜单是否能出现在角色权限配置树中。
2. 禁用菜单是否从授权树中隐藏。
3. 删除菜单前是否需要检查 `sys_role_permission` 引用。
4. 菜单权限标识和权限点是否需要自动同步。

第一版建议：菜单管理只维护 `sys_menu`，权限点仍由 `sys_permission` 独立维护或种子数据维护。后续再评估是否在新增菜单时自动创建权限点。

## 7. 前端页面建议

路由：

```text
/system/menu
```

页面能力：

1. 菜单树表格展示。
2. 关键词、状态、类型筛选。
3. 新增根菜单。
4. 新增子菜单。
5. 编辑菜单。
6. 启停菜单。
7. 删除菜单。

展示字段：

1. 菜单名称
2. 类型
3. 路由路径
4. 权限标识
5. 图标
6. 排序
7. 是否显示
8. 状态
9. 操作

表单字段：

1. 上级菜单
2. 菜单类型
3. 菜单名称
4. 路由名称
5. 路由路径
6. 组件路径
7. 权限标识
8. 图标
9. 排序
10. 显示状态
11. 缓存状态
12. 外链状态
13. 启用状态
14. 备注

视觉要求：

1. 复用 `PageContent`、`PageSearch`、`BaseTable`、`PageModal`。
2. 保持与用户管理、角色管理页面一致的后台管理质感。
3. 树表格的操作列宽度预留充足，避免横向滚动时挤压按钮。
4. 第一版不要做过度复杂的图标选择器，可以先用输入框维护图标名。

## 8. APIFox 文档建议

建议新增：

```text
docs/apifox-system-menu.openapi.json
```

覆盖接口：

1. `GET /api/system/menus/tree`
2. `GET /api/system/menus/{id}`
3. `POST /api/system/menus`
4. `PATCH /api/system/menus/{id}`
5. `PATCH /api/system/menus/{id}/status`
6. `DELETE /api/system/menus/{id}`

联调账号：

```text
admin / Admin@123456
```

## 9. 分阶段实施顺序

### 阶段一：后端基础接口

1. 新增 `system_menu` 模块。
2. 实现菜单树查询。
3. 实现菜单详情。
4. 实现新增和编辑。
5. 实现启停和删除。
6. 补路由注册。

### 阶段二：前端页面

1. 新增 `frontend/src/api/system-menu.ts`。
2. 新增 `frontend/src/views/system/menu/MenuManagementView.vue`。
3. 更新路由从建设中页面切换到真实页面。
4. 接入菜单树表格和弹窗表单。
5. 完成基础增删改查联调。

### 阶段三：联动回归

1. 修改菜单后验证 `/api/auth/menus` 返回变化。
2. 修改菜单后验证角色权限配置树变化。
3. 验证禁用菜单后侧边栏不展示。
4. 验证删除菜单前的引用保护。

### 阶段四：文档与 APIFox

1. 补 OpenAPI 导入文件。
2. 补 APIFox 调试说明。
3. 将接口样例写入文档索引。

## 10. 当前分支验收标准

本分支完成后应满足：

1. `/system/menu` 不再显示建设中页面。
2. 管理员可以查看菜单树。
3. 管理员可以新增、编辑、启停、删除菜单。
4. 菜单删除有子节点或授权引用时会被保护。
5. 前端构建和类型检查通过。
6. 后端格式检查和 Docker 构建通过。
7. APIFox 文档可以导入并覆盖菜单管理接口。

## 11. 相关文档

1. [动态菜单初始化方案](./dynamic-menu-init-plan.md)
2. [菜单接口设计](./auth-menus-api-design.md)
3. [后端菜单实现说明](./backend-auth-menus-implementation-plan.md)
4. [角色权限配置接口设计](./role-permission-config-api-design.md)
5. [用户角色绑定真实化执行清单](./user-role-assignment-plan.md)
