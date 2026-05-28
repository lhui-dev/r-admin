# 后端初始化计划

## 1. 目标

本阶段目标是在 `feature/init-backend` 分支上完成 Rust 后端工程骨架初始化，为后续认证、RBAC、菜单、日志与 PostgreSQL 接入提供基础运行能力。

## 2. 模板来源

后端模板沿用：

- [lhui-dev/rct](https://github.com/lhui-dev/rct)

## 3. 模板检查结论

根据当前模板仓库内容，`rct` 更偏向“Rust 工程化模板”，而不是现成的 Web 后端脚手架。

当前模板已提供：

1. `Cargo.toml` 基础工程元信息
2. `Makefile.toml` 工程任务流
3. `cargo-generate.toml` 模板变量配置
4. `deny.toml`、`cliff.toml`、`typos.toml` 等工程规范文件
5. `.github/` 自动化基础目录

当前模板未提供：

1. Axum Web 服务骨架
2. PostgreSQL 连接与迁移配置
3. API 路由结构
4. JWT、认证中间件、统一响应与错误处理

因此本项目采用的策略是：

`继承 rct 的工程化底座，再补齐管理系统后端所需的 Web/API 能力。`

## 4. 初始化策略

建议采用以下方式使用模板：

1. 在 `backend/` 中沿用 `rct` 的工程规范文件与基础组织方式
2. 保留其工程化工具链思路，如 `cargo-make`、`cargo-deny`、`typos`
3. 将项目改造为面向 `Axum + SQLx + PostgreSQL` 的 API 服务
4. 调整包名、仓库地址、描述信息，使其匹配 `r-admin` 项目

## 5. 第一批落地内容

`feature/init-backend` 第一批建议只做这些：

1. 初始化 `backend/Cargo.toml`
2. 引入基础依赖：`axum`、`tokio`、`sqlx`、`serde`、`tracing` 等
3. 建立 `src/main.rs`
4. 建立基础模块目录：
   - `src/config/`
   - `src/common/`
   - `src/routes/`
   - `src/state/`
   - `src/modules/`
   - `src/middleware/`
5. 建立应用配置加载
6. 建立数据库连接池
7. 提供 `/api/health` 健康检查接口
8. 提供统一 JSON 响应结构
9. 对齐 Docker 后端构建入口

## 6. 本阶段暂不处理

为了保证初始化阶段可控，以下内容建议放到后续分支：

1. 登录接口
2. JWT 认证
3. RBAC 鉴权中间件
4. 用户、角色、权限 CRUD
5. Redis
6. 多租户
7. 数据权限

## 7. 完成标准

当以下条件满足时，可以认为 `feature/init-backend` 第一阶段完成：

1. `backend/` 工程可编译
2. 本地可启动服务
3. `/api/health` 返回成功
4. PostgreSQL 连接可初始化
5. Docker 后端镜像可构建

## 8. 下一步

本计划确认后，后续最直接的动作就是：

1. 将 `rct` 模板规范文件映射到 `backend/`
2. 初始化 `backend/Cargo.toml`
3. 搭建 `Axum + SQLx` 最小运行骨架
