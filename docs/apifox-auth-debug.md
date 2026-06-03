# APIFox 联调说明

本文档用于说明如何把当前项目的后端接口快速导入 APIFox，并在后续接口迭代时保持调试资产同步更新。

当前这份文档已经覆盖：

1. 健康检查
2. 认证接口
3. 用户管理第一版接口
4. 角色权限配置接口

## 1. 导入文件

当前提供的 APIFox 相关文件：

1. [OpenAPI 导入文件](./apifox-auth.openapi.json)
2. [角色权限配置 OpenAPI 导入文件](./apifox-role-permission.openapi.json)
3. [APIFox 调试脚本清单](./apifox-debug-scripts.md)

当前 OpenAPI 文件覆盖：

1. `GET /api/health`
2. `POST /api/auth/login`
3. `GET /api/auth/me`
4. `GET /api/auth/menus`
5. `POST /api/auth/logout`
6. `GET /api/system/users`
7. `GET /api/system/users/{id}`
8. `POST /api/system/users`
9. `PATCH /api/system/users/{id}`
10. `PATCH /api/system/users/{id}/status`
11. `GET /api/system/roles/{id}/permission-config`
12. `PUT /api/system/roles/{id}/permissions`

## 2. 导入方式

建议在 APIFox 中按以下方式导入：

1. 进入项目
2. 选择“导入”
3. 选择 `OpenAPI/Swagger`
4. 选择文件 [apifox-auth.openapi.json](./apifox-auth.openapi.json)
5. 导入后按 `Health`、`Auth`、`System User` 三个分组检查接口是否完整

如果你准备单独调“角色权限配置”这一条线，也可以直接导入：

1. [apifox-role-permission.openapi.json](./apifox-role-permission.openapi.json)
2. 导入后检查 `System Role Permission` 分组

如果你准备继续在 APIFox 中挂前后置脚本，建议同时参考：

1. [APIFox 调试脚本清单](./apifox-debug-scripts.md)

## 3. 推荐环境变量

建议在 APIFox 中创建一个本地调试环境：

```text
baseUrl = http://127.0.0.1:8080
access_token =
login_username = admin
login_password = Admin@123456
expected_role = super_admin
```

如果你需要临时避开端口冲突，也可以加一个备用环境：

```text
baseUrl = http://127.0.0.1:18080
access_token =
login_username = admin
login_password = Admin@123456
expected_role = super_admin
```

说明：

1. `8080` 是默认本地后端端口
2. `18080` 是之前做临时联调时使用过的备用端口
3. 当前所有需要登录的接口都复用同一个 `access_token`

## 4. 当前默认调试账号

当前种子数据默认测试账号为：

```text
username: admin
password: Admin@123456

username: sysadmin
password: Admin@123456

username: auditor
password: Admin@123456
```

对应 SQL 文件：

1. [种子数据脚本](../sql/seed-data.sql)

当前基础部门与角色示例：

```text
dept: 100 平台总部
dept: 110 技术中心
dept: 120 运营中心

role: 300 super_admin
role: 310 system_admin
role: 320 auditor
role: 330 operator
```

## 5. 推荐调试顺序

建议按下面顺序调试：

1. `GET /api/health`
2. `POST /api/auth/login`
3. `GET /api/auth/me`
4. `GET /api/auth/menus`
5. `GET /api/system/users`
6. `POST /api/system/users`
7. `GET /api/system/users/{id}`
8. `PATCH /api/system/users/{id}`
9. `PATCH /api/system/users/{id}/status`
10. `POST /api/auth/logout`
11. `GET /api/system/roles/310/permission-config`
12. `PUT /api/system/roles/310/permissions`

这样做的原因：

1. 先确认服务和数据库状态
2. 再确认登录与鉴权链路正常
3. 最后再调需要登录态的业务接口

## 6. 脚本复用建议

当前用户管理接口不需要新增独立鉴权脚本，直接复用：

1. [通用鉴权前置脚本](./apifox-scripts/auth-prerequest.js)
2. [登录前置脚本](./apifox-scripts/login-prerequest.js)
3. [登录后置脚本](./apifox-scripts/login-postresponse.js)

建议：

1. 在 `POST /api/auth/login` 执行后自动保存 `access_token`
2. 在所有 `/api/system/users*` 接口前统一挂 `auth-prerequest.js`
3. 用户管理接口如果需要做结构断言，可直接在 APIFox 内补最小后置脚本

一个最小用户列表断言示例：

```javascript
pm.test('user list status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('user list business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('user list payload shape is valid', function () {
  pm.expect(json.data.items).to.be.an('array');
  pm.expect(json.data.pagination).to.be.an('object');
});
```

## 7. 当前接口行为说明

### 7.1 `GET /api/health`

用途：

1. 检查后端服务是否启动
2. 检查数据库是否可连接

### 7.2 `POST /api/auth/login`

用途：

1. 校验用户名和密码
2. 生成 JWT Access Token
3. 返回当前用户基础信息

当前特点：

1. 只做第一阶段最小登录闭环
2. 暂未引入 Refresh Token
3. 暂未接入验证码、锁定策略、多租户登录隔离

### 7.3 `GET /api/auth/me`

用途：

1. 基于 Bearer Token 获取当前用户
2. 返回用户基础信息
3. 返回角色编码列表
4. 返回权限编码列表

### 7.4 `GET /api/auth/menus`

用途：

1. 基于 Bearer Token 返回当前用户可见菜单树
2. 返回前端可直接渲染的分组和菜单项
3. 用于前端登录后的动态菜单初始化

### 7.5 `POST /api/auth/logout`

当前阶段说明：

1. 当前实现为占位接口
2. 主要用于前端调试退出流程
3. 目前不做服务端 token 失效和黑名单管理

### 7.6 `GET /api/system/users`

用途：

1. 返回分页用户列表
2. 支持 `keyword / dept_id / status` 筛选
3. 列表中直接带轻量部门、角色、岗位摘要

当前查询参数：

1. `page`
2. `page_size`
3. `keyword`
4. `dept_id`
5. `status`

### 7.7 `GET /api/system/users/{id}`

用途：

1. 返回单个用户详情
2. 返回部门、角色、岗位摘要
3. 返回登录相关字段和资料字段

### 7.8 `POST /api/system/users`

用途：

1. 创建新用户
2. 当前支持基础资料、部门、状态入参
3. 当前不在这个接口里分配角色和岗位

当前主要校验点：

1. `username` 不能为空
2. `password` 不能为空
3. `nickname` 不能为空
4. `status` 只能是 `0 / 1`
5. `dept_id` 必须存在
6. `username / mobile / email` 不能冲突

### 7.9 `PATCH /api/system/users/{id}`

用途：

1. 更新第一版可编辑资料
2. 当前不允许修改 `username`
3. 当前不允许修改 `password`
4. 当前不允许修改 `status`

当前可更新字段：

1. `nickname`
2. `real_name`
3. `mobile`
4. `email`
5. `gender`
6. `dept_id`
7. `remark`

### 7.10 `PATCH /api/system/users/{id}/status`

用途：

1. 启用或停用用户
2. 当前独立拆分成单一职责接口

当前限制：

1. `status` 只能是 `0 / 1`
2. 不能停用超级管理员
3. 不能停用当前登录用户自己

### 7.11 `GET /api/system/roles/{id}/permission-config`

用途：

1. 返回角色摘要
2. 返回完整权限树
3. 返回当前角色已勾选的叶子权限集合

当前联调建议直接使用：

1. `role id = 310`
2. 对应角色编码 `system_admin`

### 7.12 `PUT /api/system/roles/{id}/permissions`

用途：

1. 以全量覆盖方式保存当前角色权限
2. 保存成功后返回最新角色摘要和权限摘要

当前说明：

1. 请求体使用 `permission_ids`
2. 建议先用种子权限全集回写，避免把本地角色长期停留在临时测试状态

## 8. 统一回归结果

本次联调与回归覆盖了：

1. OpenAPI 文档结构补全
2. OpenAPI JSON 可解析性校验
3. 后端编译检查
4. 用户管理真实接口链路回归
5. 角色权限配置真实接口链路回归

当前已验证通过的接口：

1. `GET /api/health`
2. `POST /api/auth/login`
3. `GET /api/system/users`
4. `GET /api/system/users/{id}`
5. `POST /api/system/users`
6. `PATCH /api/system/users/{id}`
7. `PATCH /api/system/users/{id}/status`
8. `GET /api/system/roles/310/permission-config`
9. `PUT /api/system/roles/310/permissions`

本次真实联调用例：

1. 使用 `admin / Admin@123456` 登录获取 token
2. 创建用户 `operator192830`
3. 创建成功后拿到 `id = 1050`
4. 查询列表确认能按 `keyword=operator192830` 搜到
5. 查询详情确认初始部门为 `技术中心`
6. 更新昵称与部门到 `运营中心`
7. 停用该用户，状态变更为 `0`
8. 再查详情确认更新已生效
9. 通过前端 `/api` 代理打通 `role permission config` 查询与保存接口

## 9. 返回数据示例

### 9.1 `GET /api/health`

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "status": "ok",
    "service": "r-admin-backend",
    "database": "up"
  }
}
```

### 9.2 `POST /api/auth/login`

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "access_token": "<jwt-token>",
    "token_type": "Bearer",
    "expires_in": 7200,
    "user": {
      "id": 1000,
      "username": "admin",
      "nickname": "超级管理员",
      "real_name": "平台管理员",
      "is_super_admin": true
    }
  }
}
```

### 9.3 `GET /api/system/users?page=1&page_size=20&keyword=operator192830`

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "items": [
      {
        "id": 1050,
        "username": "operator192830",
        "nickname": "操作员联调-更新",
        "real_name": "联调测试用户",
        "mobile": "139192830",
        "email": "operator192830@example.com",
        "status": 0,
        "is_super_admin": false,
        "dept": {
          "id": 120,
          "name": "运营中心"
        },
        "roles": [],
        "posts": [],
        "last_login_at": null,
        "created_at": "2026-06-03T11:28:30Z"
      }
    ],
    "pagination": {
      "page": 1,
      "page_size": 20,
      "total": 1
    }
  }
}
```

### 9.4 `GET /api/system/users/1050`

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1050,
    "username": "operator192830",
    "nickname": "操作员联调-更新",
    "real_name": "联调测试用户",
    "mobile": "139192830",
    "email": "operator192830@example.com",
    "avatar_url": null,
    "gender": 1,
    "status": 0,
    "is_super_admin": false,
    "remark": "updated by live regression",
    "dept": {
      "id": 120,
      "name": "运营中心"
    },
    "roles": [],
    "posts": [],
    "last_login_at": null,
    "last_login_ip": null,
    "password_updated_at": "2026-06-03T11:28:30Z",
    "created_at": "2026-06-03T11:28:30Z",
    "updated_at": "2026-06-03T11:28:31Z"
  }
}
```

### 9.5 `POST /api/system/users`

请求体示例：

```json
{
  "username": "operator192830",
  "password": "Admin@123456",
  "nickname": "操作员联调",
  "real_name": "联调测试用户",
  "mobile": "139192830",
  "email": "operator192830@example.com",
  "gender": 1,
  "dept_id": 110,
  "status": 1,
  "remark": "APIFox live regression user"
}
```

返回示例：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1050,
    "username": "operator192830"
  }
}
```

### 9.6 `PATCH /api/system/users/1050`

请求体示例：

```json
{
  "nickname": "操作员联调-更新",
  "real_name": "联调测试用户",
  "mobile": "139192830",
  "email": "operator192830@example.com",
  "gender": 1,
  "dept_id": 120,
  "remark": "updated by live regression"
}
```

返回示例：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1050,
    "username": "operator192830"
  }
}
```

### 9.7 `PATCH /api/system/users/1050/status`

请求体示例：

```json
{
  "status": 0
}
```

返回示例：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 1050,
    "username": "operator192830",
    "status": 0
  }
}
```

### 9.8 `GET /api/system/roles/310/permission-config`

返回示例：

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
      "data_scope": "department",
      "sort": 10,
      "is_builtin": true,
      "user_count": 1,
      "permission_count": 27,
      "remark": "系统内置系统管理员角色",
      "created_at": "2026-05-31T14:43:34Z",
      "updated_at": "2026-06-03T14:48:16Z"
    },
    "permission_tree": [
      {
        "id": "module:21000",
        "name": "系统管理",
        "type": "module",
        "children": []
      }
    ],
    "checked_permission_ids": [
      "dashboard:view",
      "system:role:assign-permission",
      "system:role:list"
    ]
  }
}
```

### 9.9 `PUT /api/system/roles/310/permissions`

请求体示例：

```json
{
  "permission_ids": [
    "dashboard:view",
    "system:user:list",
    "system:user:create",
    "system:user:update",
    "system:user:reset-password",
    "system:user:assign-role",
    "system:role:list",
    "system:role:create",
    "system:role:update",
    "system:role:assign-permission",
    "system:menu:list",
    "system:menu:create",
    "system:menu:update",
    "system:dept:list",
    "system:dept:create",
    "system:dept:update",
    "system:post:list",
    "system:post:create",
    "system:post:update",
    "system:dict:list",
    "system:dict:create",
    "system:dict:update",
    "system:config:list",
    "system:config:create",
    "system:config:update",
    "system:log:login:list",
    "system:log:operation:list"
  ]
}
```

返回示例：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": 310,
    "code": "system_admin",
    "name": "系统管理员",
    "status": 1,
    "data_scope": "department",
    "sort": 10,
    "is_builtin": true,
    "user_count": 1,
    "permission_count": 27,
    "remark": "系统内置系统管理员角色",
    "created_at": "2026-05-31T14:43:34Z",
    "updated_at": "2026-06-03T15:00:08Z",
    "permissions": [
      {
        "id": "dashboard:view",
        "name": "首页查看"
      },
      {
        "id": "system:role:assign-permission",
        "name": "角色分配权限"
      }
    ]
  }
}
```

## 10. 后续新增接口时如何维护

后续新增接口后，建议始终同步维护这份 OpenAPI 文件，保持 APIFox 调试入口稳定。

建议维护规则：

1. 新增后端路由时，同步更新 [apifox-auth.openapi.json](./apifox-auth.openapi.json)
2. 角色权限配置这类独立链路，允许拆成单独导入文件，例如 [apifox-role-permission.openapi.json](./apifox-role-permission.openapi.json)
3. 优先保持 `tags`、`summary`、`requestBody`、`parameters`、`responses` 完整
4. 如果接口需要登录，补上 `BearerAuth`
5. 如果接口返回结构有复用，优先补到 `components.schemas`
6. 错误响应尽量补充 `400`、`401`、`403`、`404`、`409`
7. 联调通过后，把一个真实成功示例同步写回文档

## 11. 推荐的后续拆分方式

为了方便后续维护，建议后面不要一直把所有接口都堆在一个文件里。

推荐演进方式：

1. 当前阶段先保留一个总入口文件：`docs/apifox-auth.openapi.json`
2. 后续接口增多后，拆成多个文件：
   `docs/openapi/auth.openapi.json`
   `docs/openapi/system-user.openapi.json`
   `docs/openapi/system-role.openapi.json`
   `docs/openapi/system-menu.openapi.json`
3. 如果未来后端引入 `utoipa` 或其他 OpenAPI 自动生成方案，再逐步切换成“代码生成文档”

一句话原则：

`接口变更和调试文档变更尽量同提交。`
