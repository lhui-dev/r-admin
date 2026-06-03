# 角色权限配置接口设计

## 1. 目标

本文档用于定义“角色权限配置”这一块的前后端接口契约，作为当前前端原型页与后续 backend 实现之间的统一基线。

当前目标不是一次性做完完整权限中心，而是先把下面这条链路定稳：

1. 进入角色权限配置页
2. 拉取当前角色摘要
3. 拉取可配置权限树
4. 返回当前角色已勾选的权限点
5. 提交新的权限点集合并完成保存

一句话目标：

`先把“角色拿到什么权限、如何查看、如何保存”这条主链路定稳，再继续扩展数据权限、批量授权和更复杂的联动规则。`

## 2. 当前基础

当前项目已经具备：

1. 角色管理前端原型页
2. 角色权限配置前端原型页
3. `sys_role`
4. `sys_permission`
5. `sys_role_permission`
6. `sys_menu.permission_code` 与权限标识的逻辑映射

当前前端原型已经在使用以下概念：

1. 角色摘要
2. 权限树
3. 已选权限点 ID 集合
4. 保存后更新时间

这意味着第一版后端接口完全可以直接围绕现有库表与前端页面结构落地。

## 3. 第一版范围

第一版建议只纳入两类接口：

1. 角色权限配置详情查询
2. 角色权限配置保存

第一版暂不建议纳入：

1. 数据权限范围单独配置
2. 菜单树拖拽排序
3. 按模块批量复制角色权限
4. 角色权限差异对比
5. 批量角色授权
6. 生效预览和授权审计

一句话原则：

`先把单角色的“查配置、改配置、存配置”跑通，再继续叠加更复杂的授权运营能力。`

## 4. 推荐接口清单

第一版建议优先定义：

1. `GET /api/system/roles/{id}/permission-config`
2. `PUT /api/system/roles/{id}/permissions`

说明：

1. 第一个接口负责初始化页面所需全部数据
2. 第二个接口负责以“最终权限集合”方式覆盖保存

这里建议保存接口用 `PUT`，因为当前前端交互本质上是“提交一份完整权限点集合”，语义更接近整体替换，而不是局部 patch。

## 5. 查询接口设计

### 5.1 路径

```text
GET /api/system/roles/{id}/permission-config
```

### 5.2 返回结构建议

建议保持当前项目统一响应格式：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "role": {
      "id": 310,
      "code": "system_admin",
      "name": "系统管理员",
      "status": 1,
      "data_scope": "tenant",
      "sort": 10,
      "is_builtin": true,
      "user_count": 6,
      "permission_count": 18,
      "remark": "负责租户内系统配置与授权维护",
      "created_at": "2026-06-01T09:00:00+08:00",
      "updated_at": "2026-06-03T14:30:00+08:00"
    },
    "permission_tree": [
      {
        "id": "module:system",
        "name": "系统管理",
        "type": "module",
        "children": [
          {
            "id": "menu:system:role",
            "name": "角色管理",
            "type": "menu",
            "children": [
              {
                "id": "system:role:list",
                "name": "查看角色列表",
                "type": "button"
              },
              {
                "id": "system:role:permission",
                "name": "配置角色权限",
                "type": "button"
              }
            ]
          }
        ]
      }
    ],
    "checked_permission_ids": [
      "system:role:list",
      "system:role:permission"
    ]
  }
}
```

### 5.3 为什么查询接口一次返回完整页面数据

建议详情初始化时一次返回 `role + permission_tree + checked_permission_ids`，原因：

1. 前端进入页面后不需要再串多次请求
2. 页面右侧摘要和左侧树结构天然属于同一上下文
3. 后续接入真实接口时可以最小成本替换当前 mock

## 6. 保存接口设计

### 6.1 路径

```text
PUT /api/system/roles/{id}/permissions
```

### 6.2 请求体建议

```json
{
  "permission_ids": [
    "system:role:list",
    "system:role:permission",
    "system:user:list"
  ]
}
```

### 6.3 保存语义建议

建议第一版采用“全量覆盖”语义：

1. 前端提交当前勾选的最终叶子权限集合
2. 后端在事务中删除该角色已有权限关联
3. 再插入新的 `sys_role_permission`

这样可以保持前后端心智一致：

`页面当前勾选了什么，保存后数据库里就应该是什么。`

### 6.4 返回结构建议

建议保存成功后返回最新角色摘要，至少包含前端当前已展示字段：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 310,
    "code": "system_admin",
    "name": "系统管理员",
    "status": 1,
    "data_scope": "tenant",
    "sort": 10,
    "is_builtin": true,
    "user_count": 6,
    "permission_count": 19,
    "remark": "负责租户内系统配置与授权维护",
    "created_at": "2026-06-01T09:00:00+08:00",
    "updated_at": "2026-06-03T15:10:00+08:00",
    "permissions": [
      {
        "id": "system:role:list",
        "name": "查看角色列表"
      },
      {
        "id": "system:role:permission",
        "name": "配置角色权限"
      }
    ]
  }
}
```

说明：

1. 当前前端保存后会刷新页面中的角色摘要和已选标签
2. 因此建议返回最新角色详情，而不是只回一个 `success`

## 7. 权限树节点字段建议

建议第一版树节点至少包含：

```json
{
  "id": "system:role:permission",
  "name": "配置角色权限",
  "type": "button",
  "children": []
}
```

字段说明：

1. `id`
   当前前端直接把它当作树节点主键和已选权限点标识
2. `name`
   当前前端直接用于树节点展示
3. `type`
   当前前端直接用于显示“模块 / 菜单 / 按钮 / 接口”标签
4. `children`
   有子节点时返回数组，无子节点时建议省略或返回空数组

## 8. 前端当前已直接消费的字段

下面这些字段已经被当前前端权限配置页直接使用，后端实现时建议优先保持一致：

### 8.1 `data.role`

当前页面直接依赖：

1. `id`
2. `code`
3. `name`
4. `status`
5. `data_scope`
6. `is_builtin`
7. `permission_count`
8. `updated_at`
9. `remark`

### 8.2 `data.permission_tree`

当前页面直接依赖：

1. `id`
2. `name`
3. `type`
4. `children`

### 8.3 `data.checked_permission_ids`

当前页面直接依赖：

1. 勾选树节点初始化
2. 右侧已选标签
3. 未保存变更判断
4. 覆盖率统计

一句话结论：

`后端第一版只要稳定返回 role / permission_tree / checked_permission_ids，这个页面就可以从 mock 平滑切到真实接口。`

## 9. 当前 mock 与未来真实接口的边界

当前前端 mock 已经尽量贴近未来接口，但仍有几处属于原型约定：

1. `permission_tree.id` 当前直接使用权限编码或组合字符串
2. 模块节点、菜单节点和按钮节点都混在同一棵树中
3. 保存后直接回最新角色详情，而不是重新查询详情接口

这些约定是可以接受的，因为它们与真实后端实现并不冲突。

当前建议后端保持：

1. 叶子权限节点 ID 稳定唯一
2. `checked_permission_ids` 仅返回真正需要授权的叶子权限 ID
3. 模块和菜单节点只承担结构展示职责

## 10. DTO 设计建议

建议在 backend 侧预留：

```text
backend/src/modules/system_role/
├─ dto.rs
├─ handler.rs
├─ service.rs
└─ mod.rs
```

建议第一版至少包括：

1. `GetRolePermissionConfigResponse`
2. `RolePermissionConfigData`
3. `RolePermissionTreeNode`
4. `UpdateRolePermissionsRequest`
5. `RolePermissionSummary`
6. `RolePermissionMutationResponse`

如果想把 DTO 再拆清楚一些，可以进一步拆成：

1. `RolePermissionConfigRole`
2. `RolePermissionTreeNodeType`
3. `RolePermissionCheckedIds`

## 11. 查询实现建议

查询接口建议按下面思路实现：

1. 从 `sys_role` 查询角色基础信息
2. 从 `sys_role_permission` 查出当前角色已有权限 ID
3. 从 `sys_permission` 查权限节点基础信息
4. 结合 `sys_menu.permission_code` 或菜单配置构建树形结构

如果第一版后端还没有完整菜单树服务，也可以先：

1. 基于 `sys_menu` 构建菜单层
2. 基于 `sys_permission` 挂接按钮或接口权限
3. 在 service 层统一转成前端所需树结构

## 12. 保存实现建议

保存接口建议：

1. 先校验角色是否存在
2. 校验提交的 `permission_ids` 是否全部存在
3. 对内置关键角色可追加保护规则
4. 使用事务更新 `sys_role_permission`
5. 保存成功后回查一次角色最新摘要

如果后续要增强审计能力，建议在这里顺手记录：

1. 操作者用户 ID
2. 角色 ID
3. 旧权限集合
4. 新权限集合
5. 变更时间

## 13. 错误语义建议

建议保持与当前项目统一风格：

1. 参数错误：`400`
2. 未登录或 token 无效：`401`
3. 无权限执行角色授权：`403`
4. 角色不存在：`404`
5. 提交了不存在的权限 ID：`409`
6. 服务内部异常：`500`

## 14. APIFox 与 OpenAPI 维护建议

后续新增这组接口后，建议同步补齐：

1. `docs/apifox-auth.openapi.json` 的下一版或拆分后的角色 OpenAPI 文件
2. 示例请求：
   `GET /api/system/roles/{id}/permission-config`
3. 示例请求：
   `PUT /api/system/roles/{id}/permissions`
4. 至少一组成功返回示例
5. 至少一组角色不存在和权限 ID 非法的失败示例

这样后续接口联调时，权限配置页也能和认证、菜单、用户管理一样走稳定的 APIFox 入口。

## 15. 第一版完成标准

当下面条件满足时，可认为角色权限配置接口已经具备进入真实编码的条件：

1. 查询接口返回结构明确
2. 保存接口请求体与返回结构明确
3. 权限树节点字段边界明确
4. 当前前端已依赖字段清单明确
5. backend DTO 与 service 落位建议明确

## 16. 推荐下一步

建议后续按下面顺序推进：

1. 在 backend 中补 `system_role` 下的权限配置 DTO
2. 先落 handler 占位与返回 mock
3. 再接数据库查询角色与权限关联
4. 最后补保存事务与审计记录

一句话总结：

`角色权限配置是 RBAC 真正进入授权核心的一步，先把接口契约和页面依赖字段定稳，后面接真实后端会顺很多。`
