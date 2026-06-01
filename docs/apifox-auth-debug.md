# APIFox 认证调试说明

本文档用于说明如何把当前项目的认证接口快速导入 APIFox，并在后续接口迭代时保持调试资产同步更新。

## 1. 导入文件

当前提供的 APIFox 导入文件：

1. [认证接口 OpenAPI 导入文件](./apifox-auth.openapi.json)
2. [APIFox 调试脚本清单](./apifox-debug-scripts.md)

该文件当前覆盖：

1. `GET /api/health`
2. `POST /api/auth/login`
3. `GET /api/auth/me`
4. `GET /api/auth/menus`
5. `POST /api/auth/logout`

## 2. 导入方式

建议在 APIFox 中按以下方式导入：

1. 进入项目
2. 选择“导入”
3. 选择 `OpenAPI/Swagger`
4. 选择文件 [apifox-auth.openapi.json](./apifox-auth.openapi.json)
5. 导入后按 `Auth`、`Health` 分组检查接口是否完整

如果你准备直接在 APIFox 中挂前置脚本和后置脚本，建议同时参考：

1. [APIFox 调试脚本清单](./apifox-debug-scripts.md)

## 3. 推荐环境变量

建议在 APIFox 中创建一个本地调试环境，例如：

```text
baseUrl = http://127.0.0.1:8080
access_token =
```

后续如果走 Docker 联调，也可以再补一个环境：

```text
baseUrl = http://127.0.0.1:18080
access_token =
```

说明：

1. `18080` 是之前为了避开本地端口冲突做联调时使用过的备用端口
2. 如果你当前仍使用默认后端端口，则优先使用 `8080`

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

注意：

1. 这次已经把种子文件中的默认密码改成真实 Argon2 哈希
2. 如果你的数据库之前已经导入过旧版本种子，数据库中的 `sys_user.password_hash` 不会自动更新
3. 如果登录失败，请优先重新执行种子 SQL，或单独更新对应账号的密码哈希

## 5. 建议调试顺序

建议按下面顺序进行：

1. 先调 `GET /api/health`
2. 再调 `POST /api/auth/login`
3. 取回 `access_token`
4. 再调 `GET /api/auth/me`
5. 再调 `GET /api/auth/menus`
6. 最后调 `POST /api/auth/logout`

## 6. 当前接口行为说明

### 6.1 `GET /api/health`

用途：

1. 检查后端服务是否启动
2. 检查数据库是否可连接

### 6.2 `POST /api/auth/login`

用途：

1. 校验用户名和密码
2. 生成 JWT Access Token
3. 返回当前用户基础信息

当前特点：

1. 只做第一阶段最小登录闭环
2. 暂未引入 Refresh Token
3. 暂未接入验证码、锁定策略、多租户登录隔离

### 6.3 `GET /api/auth/me`

用途：

1. 基于 Bearer Token 获取当前用户
2. 返回用户基础信息
3. 返回角色编码列表
4. 返回权限编码列表

### 6.4 `GET /api/auth/menus`

用途：

1. 基于 Bearer Token 返回当前用户可见菜单树
2. 返回前端可直接渲染的分组和菜单项
3. 用于前端登录后的动态菜单初始化

当前特点：

1. 当前返回结构固定在 `data.menus`
2. 一级目录菜单返回 `children`
3. 当前前端已存在页面会映射到真实路由
4. 其余菜单项会统一落到 `placeholder` 占位页

### 6.5 `POST /api/auth/logout`

当前阶段说明：

1. 当前实现为占位接口
2. 主要用于前端调试退出流程
3. 目前不做服务端 token 失效和黑名单管理

## 7. 当前联调结论

截至 2026-06-01，本地 Docker 联调已确认：

1. `GET /api/health` 可用
2. `POST /api/auth/login` 可用
3. `GET /api/auth/me` 可用
4. `GET /api/auth/menus` 可用

当前种子数据限制：

1. `admin` 当前是超级管理员
2. `sysadmin` 当前绑定 `system_admin` 角色
3. `auditor` 当前绑定 `auditor` 角色

当前已完成的菜单联调结论：

1. `admin` 可返回完整一级菜单：`首页`、`系统管理`、`日志审计`
2. `sysadmin` 可正常通过非超级管理员链路返回菜单
3. `auditor` 只返回 `首页` 和 `日志审计`，说明菜单已按权限裁剪

## 8. 后续新增接口时如何维护

后续新增接口后，建议始终同步维护这份 OpenAPI 文件，保持 APIFox 调试入口稳定。

建议维护规则：

1. 新增后端路由时，同步更新 [apifox-auth.openapi.json](./apifox-auth.openapi.json)
2. 优先保持 `tags`、`summary`、`requestBody`、`responses` 完整
3. 如果接口需要登录，补上 `BearerAuth`
4. 如果接口返回结构有复用，优先补到 `components.schemas`
5. 错误响应尽量补充 `400`、`401`、`403`、`404`

## 9. 推荐的后续拆分方式

为了方便后续维护，建议后面不要一直把所有接口都堆在一个文件里。

推荐演进方式：

1. 当前阶段先保留一个总入口文件：`docs/apifox-auth.openapi.json`
2. 后续当接口增多后，拆成多个文件：
   `docs/openapi/auth.openapi.json`
   `docs/openapi/system-user.openapi.json`
   `docs/openapi/system-role.openapi.json`
   `docs/openapi/system-menu.openapi.json`
3. 如果未来后端引入 `utoipa` 或其他 OpenAPI 自动生成方案，再逐步切换成“代码生成文档”

## 10. 建议的更新节奏

每次接口迭代建议同时完成下面几项：

1. 更新后端路由与实现
2. 更新 OpenAPI 导入文件
3. 更新调试说明中涉及的默认参数、鉴权方式、端口或环境变量
4. 再导入 APIFox 验证一轮

一句话原则：

`接口变更和调试文档变更尽量同提交。`
