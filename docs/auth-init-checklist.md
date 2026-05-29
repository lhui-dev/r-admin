# 认证初始化执行清单

> 目标：将认证初始化计划拆成可以直接执行的任务清单，便于后续按阶段落地实现。

## 1. 执行原则

本清单默认遵循以下原则：

1. 先打通最小登录闭环
2. 先做 Access Token，不急着引入 Refresh Token
3. 先完成登录态与当前用户链路，再做动态菜单
4. 先保证前后端都能联调，再开始扩展 RBAC 业务页

## 2. 建议分阶段

建议分成两个小阶段：

1. `login + token`
2. `me + route guard`

原因：

1. 第一阶段先让用户能登录
2. 第二阶段再补当前用户与路由控制
3. 改动边界更小，联调更清晰

## 3. 第一阶段：login + token

### 3.1 后端任务

1. 新增 `backend/src/modules/auth/`
2. 新增 `dto.rs`
3. 新增 `handler.rs`
4. 新增 `service.rs`
5. 新增 `mod.rs`
6. 定义登录请求 DTO：
   - `username`
   - `password`
7. 定义登录响应 DTO：
   - `access_token`
   - `token_type`
   - `expires_in`
   - `user`
8. 新增 JWT 工具文件，例如：
   - `backend/src/common/jwt.rs`
9. 为配置增加 JWT 相关项：
   - `jwt.secret`
   - `jwt.expires_in`
10. 实现登录服务逻辑：
   - 按用户名查询用户
   - 校验用户是否存在
   - 校验用户是否启用
   - 校验密码
11. 登录成功后生成 Access Token
12. 新增接口：
   - `POST /api/auth/login`
13. 将认证路由注册到 `routes/mod.rs`

### 3.2 前端任务

1. 新增 `frontend/src/views/auth/LoginView.vue`
2. 在路由中新增 `/login`
3. 新增认证 API 文件，例如：
   - `frontend/src/api/auth.ts`
4. 新增登录请求方法
5. 新增 Token 工具，例如：
   - `frontend/src/utils/auth.ts`
6. 新增 `auth store`
7. 登录成功后：
   - 保存 Token
   - 保存用户基础信息
   - 跳转 `/dashboard`

### 3.3 第一阶段验收

1. 打开 `/login` 可正常渲染
2. 输入账号密码可发起请求
3. 登录成功后拿到 Access Token
4. Token 可被前端存储
5. 登录后成功进入控制台

## 4. 第二阶段：me + route guard

### 4.1 后端任务

1. 新增认证中间件，例如：
   - `backend/src/middleware/auth.rs`
2. 实现 JWT 解析
3. 将用户身份挂到请求上下文
4. 新增当前用户接口：
   - `GET /api/auth/me`
5. `me` 接口返回：
   - 用户基础信息
   - 角色列表
   - 权限标识列表
6. 为受保护接口预留统一鉴权入口

### 4.2 前端任务

1. Axios 自动注入 `Authorization`
2. 新增全局路由守卫
3. 未登录访问后台页时跳转 `/login`
4. 已登录访问 `/login` 时跳转 `/dashboard`
5. 页面刷新时尝试恢复登录态
6. 若本地有 Token 但无用户信息，则调用 `/api/auth/me`
7. 新增退出登录逻辑：
   - 清空 Token
   - 清空用户信息
   - 跳转 `/login`

### 4.3 第二阶段验收

1. 刷新页面后登录态仍可恢复
2. 未登录访问 `/dashboard` 会被拦截
3. `/api/auth/me` 返回当前用户信息
4. 退出登录后无法继续访问后台页

## 5. 建议接口清单

第一版建议只定义这 3 个接口：

1. `POST /api/auth/login`
2. `GET /api/auth/me`
3. `POST /api/auth/logout`

说明：

1. `logout` 第一版可以先做前端本地退出
2. 服务端 Token 失效可以放到后续增强阶段

## 6. 建议配置清单

后端建议增加：

1. `JWT_SECRET`
2. `JWT_EXPIRES_IN`

前端建议增加：

1. `VITE_API_BASE_URL`

## 7. 建议测试清单

### 7.1 后端

1. 错误用户名登录
2. 错误密码登录
3. 禁用用户登录
4. 正确账号密码登录
5. 非法 Token 访问 `/api/auth/me`
6. 合法 Token 访问 `/api/auth/me`

### 7.2 前端

1. 登录页输入校验
2. 登录失败提示
3. 登录成功跳转
4. 刷新后状态恢复
5. 未登录访问后台页跳转
6. 退出登录后状态清理

## 8. 本阶段不要急着做的事

以下内容建议先不要纳入第一版：

1. 图形验证码
2. Refresh Token
3. 登录失败次数限制
4. 密码过期策略
5. 双因素认证
6. 单设备/多设备会话管理
7. 多租户登录隔离

## 9. 完成后下一步

认证初始化完成后，建议按这个顺序继续：

1. 动态菜单初始化
2. 顶栏接入当前用户信息
3. 用户管理页
4. 角色管理页
5. 权限点管理页

一句话总结：

`先让系统真正具备“谁在登录”的能力，再让系统判断“这个人能做什么”。`
