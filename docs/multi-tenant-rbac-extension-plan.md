# 多租户 RBAC 扩展规划

本文档用于记录当前 `r-admin` 从单租户 RBAC 演进到多租户 RBAC 的设计边界、改造清单与落地顺序。当前阶段不直接改动代码和数据库结构，而是先明确后续扩展方向，避免用户、角色、权限、菜单模块继续按全局模型扩散。

## 1. 当前结论

当前 RBAC 方案适合作为多租户 RBAC 的基础，但还不是最终的多租户模型。

可以继续复用的部分：

1. `sys_user -> sys_user_role -> sys_role -> sys_role_permission -> sys_permission` 授权链路清晰，后续只需要补租户过滤和角色作用域。
2. 用户角色独立分配接口已经拆出，后续可以在 service 层增加租户边界校验。
3. 角色表已有 `data_scope` 字段，并且代码中已经出现 `tenant` 数据范围概念。
4. 后端已经具备角色有效性校验、超管保护、事务覆盖更新等基础能力。

当前不适合直接多租户化的部分：

1. 用户、角色、部门、岗位等核心表还没有 `tenant_id`。
2. `username`、`mobile`、`email` 当前是全局唯一，多租户下通常应改为租户内唯一。
3. `super_admin` 当前是全局角色概念，后续需要拆分平台级超管和租户级管理员。
4. 用户角色关系表没有租户边界，未来需要防止跨租户分配角色。
5. 当前查询接口没有从登录态中提取租户上下文，也没有强制追加租户过滤条件。

## 2. 推荐权限分层

多租户场景下不建议继续使用一个全局 `super_admin` 覆盖所有场景，应拆成平台级和租户级两层。

| 层级 | 建议角色 | 作用范围 | 能否跨租户 |
| --- | --- | --- | --- |
| 平台级 | `platform_super_admin` | 平台运营、租户管理、全局配置、全局审计 | 可以 |
| 租户级 | `tenant_admin` 或 `tenant_super_admin` | 单个租户内用户、角色、菜单、业务数据 | 不可以 |
| 业务级 | `auditor`、`operator` 等 | 单租户内业务操作 | 不可以 |

核心规则：

1. 平台级超管只能由平台侧创建和维护。
2. 租户管理员只能管理本租户内用户与角色。
3. 普通租户用户不能被分配平台级角色。
4. 平台级角色不参与普通租户的角色下拉列表。
5. 用户角色分配必须校验用户与角色是否属于同一租户。

## 3. 数据库扩展建议

### 3.1 新增租户表

建议新增 `sys_tenant` 表作为租户主数据。

建议字段：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `id` | `BIGINT` | 租户 ID |
| `tenant_name` | `VARCHAR(128)` | 租户名称 |
| `tenant_code` | `VARCHAR(64)` | 租户编码 |
| `status` | `SMALLINT` | 状态 |
| `expired_at` | `TIMESTAMPTZ` | 到期时间 |
| `created_at` | `TIMESTAMPTZ` | 创建时间 |
| `updated_at` | `TIMESTAMPTZ` | 更新时间 |
| `is_deleted` | `BOOLEAN` | 软删除 |
| `remark` | `VARCHAR(500)` | 备注 |

唯一约束建议：

1. `uk_sys_tenant_code (tenant_code)`

### 3.2 核心表增加租户字段

建议优先为以下表增加 `tenant_id`：

1. `sys_user`
2. `sys_role`
3. `sys_dept`
4. `sys_post`
5. `sys_user_role`
6. `sys_user_post`
7. `sys_role_permission`
8. `sys_dict_type`
9. `sys_dict_item`
10. `sys_config`
11. `sys_login_log`
12. `sys_operation_log`

菜单和权限是否加 `tenant_id` 需要按产品形态决定。

推荐方案：

1. `sys_permission` 先保持平台全局，表示系统能力点。
2. `sys_menu` 先保持平台全局，表示标准菜单树。
3. 租户菜单差异通过角色权限、套餐或租户授权表控制。
4. 如果未来需要租户自定义菜单，再引入 `sys_tenant_menu` 或 `tenant_id` 菜单副本。

### 3.3 唯一约束调整

当前全局唯一约束在多租户下需要调整。

| 当前约束 | 多租户建议 |
| --- | --- |
| `sys_user.username` 全局唯一 | `(tenant_id, username)` 租户内唯一 |
| `sys_user.mobile` 全局唯一 | `(tenant_id, mobile)` 租户内唯一，或平台全局唯一 |
| `sys_user.email` 全局唯一 | `(tenant_id, email)` 租户内唯一，或平台全局唯一 |
| `sys_role.role_code` 全局唯一 | `(tenant_id, role_code)` 租户内唯一 |
| `sys_dept.dept_code` 全局唯一 | `(tenant_id, dept_code)` 租户内唯一 |
| `sys_post.post_code` 全局唯一 | `(tenant_id, post_code)` 租户内唯一 |

手机号和邮箱是否全局唯一取决于登录策略：

1. 如果支持同一手机号加入多个租户，建议租户内唯一。
2. 如果手机号作为全平台登录账号，建议全局唯一，并引入用户租户关系表。

## 4. 登录态与 JWT 扩展

当前 JWT 中已有用户身份与角色信息，后续需要补充租户上下文。

建议增加：

```json
{
  "user_id": 1000,
  "tenant_id": 1,
  "tenant_code": "default",
  "roles": ["tenant_admin"],
  "is_platform_admin": false
}
```

后端接口处理规则：

1. 普通接口默认从 token 中读取 `tenant_id`。
2. 查询用户、角色、部门、岗位时必须追加 `tenant_id = current_tenant_id`。
3. 平台级接口必须显式校验 `is_platform_admin = true`。
4. 租户级管理员不能通过请求参数切换 `tenant_id`。
5. 如果未来支持一个用户加入多个租户，应先进入租户选择流程，再签发包含当前租户的 token。

## 5. 用户角色分配接口扩展

当前接口：

```http
PATCH /api/system/users/{id}/roles
```

当前请求：

```json
{
  "role_ids": [310, 330]
}
```

多租户扩展后，建议仍然保持这个接口形式，不建议前端传 `tenant_id`。

后端应在 service 层增加以下校验：

1. 目标用户必须属于当前租户。
2. 待分配角色必须属于当前租户，或属于允许租户使用的平台内置角色。
3. 租户管理员不能分配平台级角色。
4. 租户管理员不能修改平台级用户。
5. 租户超管用户必须至少保留一个租户管理员角色。
6. 平台超管角色只能由平台级接口分配。

建议将当前 `find_assignable_roles_by_ids` 扩展为：

```text
find_assignable_roles_by_ids(tx, tenant_context, role_ids)
```

并在 SQL 中追加：

```sql
AND (
    tenant_id = $current_tenant_id
    OR scope = 'platform_builtin'
)
```

最终条件是否允许平台内置角色进入租户，需要由产品策略决定。

## 6. 角色模型扩展

建议为 `sys_role` 增加角色作用域字段。

推荐字段：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `tenant_id` | `BIGINT` | 所属租户，平台角色可为空或固定为平台租户 |
| `role_scope` | `VARCHAR(32)` | `platform`、`tenant`、`builtin` |
| `is_builtin` | `BOOLEAN` | 是否内置角色，当前已有 |

建议角色编码：

1. `platform_super_admin`
2. `tenant_admin`
3. `auditor`
4. `operator`

不建议继续在多租户最终模型中使用单一 `super_admin` 表达所有权限。

## 7. 前端扩展建议

前端需要预留租户上下文，但不要过早复杂化。

建议后续新增：

1. 顶部租户标识展示。
2. 平台管理员的租户切换入口。
3. 普通租户用户不可见租户切换入口。
4. 用户管理页角色下拉只展示当前租户可分配角色。
5. 分配角色弹窗中区分内置角色和租户角色。
6. 登录成功后缓存当前租户信息。

用户管理页当前逻辑可以复用：

1. `roleOptions` 改为后端返回当前租户可分配角色。
2. `assignRoleIds` 继续只保存角色 ID。
3. 前端不直接拼接或信任 `tenant_id`，避免越权风险。

## 8. 分阶段落地计划

### 阶段一：文档与边界确认

目标：只定规则，不迁移数据。

1. 明确平台超管和租户管理员边界。
2. 明确用户是否可以加入多个租户。
3. 明确手机号、邮箱是否全局唯一。
4. 明确菜单和权限是否平台全局。
5. 明确租户套餐或租户授权是否纳入第一版。

### 阶段二：数据库迁移准备

目标：补充表结构但不大改业务。

1. 新增 `sys_tenant`。
2. 给核心业务表补 `tenant_id`。
3. 为存量数据创建默认租户。
4. 存量用户、角色、部门、岗位归属默认租户。
5. 调整唯一约束和索引。

### 阶段三：后端租户上下文

目标：所有受保护接口都能拿到当前租户。

1. JWT 增加 `tenant_id`。
2. Auth middleware 注入 `TenantContext`。
3. 用户、角色、部门、岗位查询追加租户过滤。
4. 用户角色分配增加同租户校验。
5. 平台级接口与租户级接口分离。

### 阶段四：前端租户感知

目标：页面展示和接口调用符合租户边界。

1. 顶部展示当前租户。
2. 登录态保存当前租户信息。
3. 用户管理、角色管理、权限配置页只展示当前租户数据。
4. 平台级入口只对平台管理员可见。

### 阶段五：审计与安全强化

目标：具备线上多租户安全基线。

1. 操作日志记录 `tenant_id`。
2. 登录日志记录 `tenant_id`。
3. 对跨租户访问返回 `403`，不要返回目标资源是否存在。
4. 为用户角色分配、角色权限配置增加审计日志。
5. 编写多租户越权测试用例。

## 9. 当前分支暂不建议实现的内容

当前 `feature/user-role-assignment` 分支主要目标是完成用户角色真实分配闭环，不建议在本分支混入多租户代码。

暂不建议做：

1. 修改 schema 增加 `tenant_id`。
2. 修改登录 token 结构。
3. 调整所有唯一约束。
4. 拆分 `super_admin` 角色。
5. 引入租户切换前端页面。

建议后续单独开分支：

```text
feature/multi-tenant-foundation
```

该分支再处理租户表、默认租户迁移、JWT 租户上下文和核心查询过滤。

## 10. 相关文档

1. [需求分析与数据库设计](./rbac-system-requirements.md)
2. [用户管理接口设计](./user-management-api-design.md)
3. [用户角色绑定真实化执行清单](./user-role-assignment-plan.md)
4. [角色权限配置接口设计](./role-permission-config-api-design.md)
5. [认证初始化计划](./auth-init-plan.md)
