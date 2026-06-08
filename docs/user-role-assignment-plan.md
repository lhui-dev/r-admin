# 用户角色绑定真实化执行清单

## 1. 目标

本文档用于承接当前角色模块真实化完成后的下一步工作，聚焦把“用户与角色的真实绑定关系”落到后端与前端联调链路中。

当前目标不是一次性完成完整用户中心，而是先把下面这条主链路打通：

1. 查询用户时看到真实角色信息
2. 新增用户时可直接绑定角色
3. 编辑用户时可更新角色集合
4. 单独分配角色时可稳定写入 `sys_user_role`
5. 为后续 `me / permission / menu / route` 闭环打好基础

一句话目标：

`先把“用户拥有哪些角色”这条链路定稳，再继续推进权限、菜单与路由的统一闭环。`

## 2. 当前基础

当前项目已经具备：

1. 用户管理前端原型页
2. 用户管理基础后端接口
3. 角色管理真实后端 CRUD
4. 角色权限配置真实接口
5. `sys_user / sys_role / sys_user_role` 等 RBAC 相关表

当前与本阶段直接相关的表：

1. `sys_user`
2. `sys_role`
3. `sys_user_role`
4. `sys_dept`
5. `sys_post`

这意味着：

1. 用户与角色关系已有数据库结构支撑
2. 用户列表与详情已经具备扩展角色返回结构的基础
3. 当前阶段可以直接进入真实接口实现与前端联调

## 3. 本阶段范围

本阶段建议纳入：

1. 用户详情真实返回角色摘要
2. 用户列表真实返回角色摘要
3. 新增用户支持角色绑定
4. 编辑用户支持角色绑定
5. 单独分配用户角色接口
6. 前端用户管理页接入真实角色选择与回填

本阶段暂不建议纳入：

1. 用户岗位分配增强
2. 用户批量导入导出
3. 更复杂的数据权限隔离
4. 用户删除恢复
5. 密码策略与首次登录改密
6. 多租户角色隔离增强

一句话原则：

`先把“用户和角色的关系维护”真实化，再继续叠加更复杂的组织、权限和安全能力。`

## 4. 推荐接口清单

本阶段建议优先补齐与调整：

1. `GET /api/system/users`
2. `GET /api/system/users/{id}`
3. `POST /api/system/users`
4. `PATCH /api/system/users/{id}`
5. `PATCH /api/system/users/{id}/roles`

说明：

1. `GET /users` 与 `GET /users/{id}` 需要稳定返回角色摘要
2. `POST /users` 与 `PATCH /users/{id}` 建议直接支持 `role_ids`
3. 单独补 `PATCH /users/{id}/roles`，便于“分配角色”弹窗或独立交互复用

## 5. 后端任务拆分

### 5.1 DTO 层

建议在 [backend/src/modules/system_user/dto.rs](../backend/src/modules/system_user/dto.rs) 中扩展：

1. `CreateUserRequest` 增加 `role_ids: Option<Vec<i64>>`
2. `UpdateUserRequest` 增加 `role_ids: Option<Vec<i64>>`
3. 新增 `UpdateUserRolesRequest`
4. 保持 `UserRoleSummary` 作为列表与详情统一角色摘要结构

### 5.2 Handler 层

建议在 [backend/src/modules/system_user/handler.rs](../backend/src/modules/system_user/handler.rs) 中增加：

1. `update_roles`

并保持现有接口职责：

1. `create` 负责用户创建与初始角色绑定
2. `update` 负责用户基础资料与角色集合覆盖更新
3. `update_roles` 负责独立分配角色

### 5.3 Service 层

建议在 [backend/src/modules/system_user/service.rs](../backend/src/modules/system_user/service.rs) 中补齐：

1. 创建用户时写入 `sys_user_role`
2. 编辑用户时覆盖更新 `sys_user_role`
3. 独立更新用户角色关系的方法
4. 角色 ID 合法性校验
5. 用户列表/详情共用的角色摘要查询方法

建议实现思路：

1. 校验 `role_ids` 是否存在、未删除、可用
2. 在事务中更新 `sys_user_role`
3. 列表与详情统一走真实角色聚合查询
4. 为后续岗位绑定复用类似实现方式预留结构

### 5.4 路由层

建议在 [backend/src/routes/mod.rs](../backend/src/routes/mod.rs) 中新增：

```text
PATCH /api/system/users/{id}/roles
```

## 6. 前端任务拆分

### 6.1 API 层

建议在用户 API 模块中补齐：

1. 新增用户提交 `role_ids`
2. 编辑用户提交 `role_ids`
3. 单独分配角色接口
4. 获取角色选择数据时复用真实角色列表接口

### 6.2 页面层

建议在用户管理页完成：

1. 新增用户表单增加角色选择
2. 编辑用户表单增加角色回填
3. 用户详情展示角色摘要
4. 用户列表展示角色信息或角色数量
5. 分配角色入口接入真实接口

### 6.3 交互层

建议补齐：

1. 支持多选角色
2. 保存后刷新列表与详情
3. 无角色时显示清晰占位文案
4. 禁用角色如允许展示，需要明确状态标识

## 7. 数据与规则建议

### 7.1 请求体建议

创建/编辑用户时建议支持：

```json
{
  "username": "operator01",
  "nickname": "操作员01",
  "dept_id": 100,
  "status": 1,
  "role_ids": [310, 330]
}
```

独立分配角色时建议：

```json
{
  "role_ids": [310, 330]
}
```

### 7.2 校验建议

建议至少校验：

1. `role_ids` 中角色必须存在
2. 角色必须未删除
3. 如已定义禁用角色不可分配，则需校验 `status`
4. 角色 ID 去重
5. 空数组是否允许，需要明确语义

### 7.3 保护规则建议

建议优先做基础保护：

1. 关键系统账号避免误移除核心角色
2. 超级管理员相关角色修改需谨慎
3. 为后续 `me / menu / permission` 的角色依赖预留保护逻辑

## 8. 联调检查项

本阶段完成后建议至少验证：

1. 新增用户时绑定角色成功
2. 编辑用户后角色即时更新
3. 用户详情正确回显角色
4. 用户列表正确展示角色摘要
5. 单独分配角色接口可稳定覆盖更新
6. 异常场景返回语义清晰

建议覆盖的异常场景：

1. 角色 ID 不存在
2. 提交空角色集合
3. 提交重复角色 ID
4. 绑定禁用角色

## 9. 推荐实施顺序

建议按下面顺序推进：

1. 后端先补 `CreateUserRequest / UpdateUserRequest` 的 `role_ids`
2. 后端补 `PATCH /api/system/users/{id}/roles`
3. 前端接入真实角色选择与回填
4. 前端补用户详情与列表角色展示
5. 做一轮完整联调

## 10. 完成标准

当下面条件满足时，可认为本阶段完成：

1. 用户新增支持角色绑定
2. 用户编辑支持角色绑定
3. 用户详情真实返回角色信息
4. 用户列表真实展示角色摘要
5. 前端用户管理页完成真实角色联调
6. 至少一轮接口与页面联调通过

## 11. 推荐下一步

当本阶段完成后，建议继续推进：

1. `me` 接口与角色关系的一致性检查
2. 菜单与权限闭环回归验证
3. `system_menu` 真实接口与配置链路

一句话总结：

`用户角色绑定真实化，是把当前 RBAC 从“角色模块单点完成”推进到“用户真正参与授权链路”的关键一步。`
