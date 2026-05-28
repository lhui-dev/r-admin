# Rust + PostgreSQL + Vue3 RBAC 项目初始化清单

> 目标：基于当前已确认的依赖选型，整理一份可直接执行的项目初始化步骤清单。
>
> 适用范围：`Rust + PostgreSQL + Vue3` 前后端分离 RBAC 管理系统。

## 1. 初始化目标

本清单用于指导项目从“文档设计阶段”进入“工程落地阶段”，重点解决以下问题：

1. 先准备什么环境。
2. 先建哪些目录和项目骨架。
3. 先安装哪些依赖。
4. 先完成哪些最小可运行能力。
5. 如何保证后续可以顺滑进入认证、RBAC、菜单、日志开发。

## 2. 推荐初始化顺序总览

建议按下面顺序推进：

1. 准备开发环境。
2. 初始化 Git 仓库规范与目录结构。
3. 初始化 PostgreSQL 数据库与执行建表脚本。
4. 初始化 Rust 后端项目。
5. 初始化 Vue3 前端项目。
6. 配置前后端基础依赖。
7. 打通后端健康检查与数据库连接。
8. 打通前端基础布局与接口请求封装。
9. 接入 OpenAPI、日志、统一错误处理。
10. 最后再进入登录、用户、角色、权限模块开发。
11. 完成基础容器化与 `docker compose` 联调。

## 3. 环境准备清单

## 3.1 基础工具

1. 安装 `Git`
2. 安装 `Rust`
3. 安装 `cargo`
4. 安装 `Node.js`
5. 安装 `pnpm` 或 `npm`
6. 安装 `PostgreSQL`
7. 安装数据库管理工具

建议版本：

1. Rust：稳定版，建议不低于 `1.80`
2. Node.js：建议 `22 LTS` 或 `24 LTS`
3. PostgreSQL：建议 `15+` 或 `16+`
4. 包管理器：优先 `pnpm`

## 3.2 本地能力确认

在初始化项目前，需要确认：

1. `rustc --version` 可正常执行
2. `cargo --version` 可正常执行
3. `node -v` 可正常执行
4. `pnpm -v` 或 `npm -v` 可正常执行
5. PostgreSQL 可正常创建数据库

## 4. 目录结构初始化

推荐根目录结构：

```text
r-admin/
├─ backend/
├─ frontend/
├─ docs/
├─ sql/
├─ docker/
├─ scripts/
├─ docker-compose.yml
├─ .editorconfig
├─ .gitignore
└─ README.md
```

说明：

1. `backend/` 放 Rust 后端工程
2. `frontend/` 放 Vue3 前端工程
3. `docs/` 放需求、ER 图、依赖选型等文档
4. `sql/` 放建表、种子数据、迁移相关脚本
5. `scripts/` 放本地启动、初始化辅助脚本
6. `docker/` 放前后端镜像构建文件与 `nginx` 配置
7. `docker-compose.yml` 作为本地联调与容器部署入口

## 5. 文档与 SQL 归档步骤

在开始工程初始化前，建议先整理已有产物：

1. 将需求文档放入 `docs/`
2. 将 E-R 图文件放入 `docs/`
3. 将 Mermaid 源码放入 `docs/`
4. 将建表 SQL 放入 `sql/`
5. 将种子数据 SQL 放入 `sql/`
6. 将 Docker 相关文件保留在根目录和 `docker/` 目录

建议纳入版本管理的文件：

1. [rbac-system-requirements.md](./rbac-system-requirements.md)
2. [rbac-er-diagram.svg](./rbac-er-diagram.svg)
3. [rbac-er-diagram.mmd](./rbac-er-diagram.mmd)
4. [dependency-selection.md](./dependency-selection.md)
5. [schema-postgresql.sql](../sql/schema-postgresql.sql)
6. [seed-data.sql](../sql/seed-data.sql)
7. [docker-compose.yml](../docker-compose.yml)
8. [dockerization.md](./dockerization.md)

## 6. 数据库初始化步骤

## 6.1 创建数据库

建议创建独立数据库，例如：

1. 数据库名：`r_admin`
2. 字符集与排序规则按 PostgreSQL 默认 UTF-8 方案

## 6.2 执行基础 SQL

执行顺序建议：

1. 执行建表脚本 `schema-postgresql.sql`
2. 执行种子数据脚本 `seed-data.sql`
3. 替换管理员密码哈希占位值
4. 当前默认从 `sql/` 目录执行与挂载 SQL，保持目录结构不变即可

## 6.3 初始化验证

执行后应确认：

1. 核心表已成功创建
2. 管理员用户 `admin` 已存在
3. `super_admin` 角色已存在
4. 权限与菜单数据已成功导入

## 7. Rust 后端初始化步骤

后端模板约束：

1. Rust 后端项目模板沿用 `lhui-dev/rct`
2. 模板来源：[lhui-dev/rct](https://github.com/lhui-dev/rct)
3. 我们后续初始化 `backend/` 时，优先继承该模板的工程结构、配置组织方式和基础开发约定
4. 在不破坏模板主干结构的前提下，再补充本项目所需的 `RBAC`、`PostgreSQL`、`JWT`、`OpenAPI`、日志审计模块

## 7.1 创建项目

建议在 `backend/` 下初始化：

1. 创建 `Cargo.toml`
2. 创建 `src/main.rs`
3. 创建基础模块目录

建议目录结构：

```text
backend/
├─ Cargo.toml
├─ .env.example
├─ migrations/
├─ src/
│  ├─ main.rs
│  ├─ app/
│  ├─ config/
│  ├─ common/
│  ├─ middleware/
│  ├─ modules/
│  ├─ routes/
│  └─ state/
└─ tests/
```

说明：

1. 如果 `lhui-dev/rct` 的实际目录结构与这里不同，初始化时以模板结构为准
2. 当前清单中的目录建议，主要用于约束我们自己的业务模块落点
3. 也就是说，模板负责“工程底座”，本清单负责“项目目标与扩展方向”

## 7.2 第一批依赖接入

建议先接入这些核心依赖：

1. `axum`
2. `tokio`
3. `sqlx`
4. `serde`
5. `serde_json`
6. `tower-http`
7. `tracing`
8. `tracing-subscriber`
9. `thiserror`
10. `anyhow`
11. `uuid`
12. `config`
13. `validator`
14. `argon2`
15. `jsonwebtoken`
16. `utoipa`

## 7.3 后端初始化最小闭环

第一阶段只要求后端达到以下结果：

1. 服务可以启动
2. 健康检查接口可访问
3. 可以读取 `.env`
4. 可以连接 PostgreSQL
5. 可以打印结构化日志
6. 可以返回统一 JSON 响应

## 7.4 后端第一批基础模块

建议先搭这些基础能力：

1. 配置加载模块
2. 应用状态 `AppState`
3. 数据库连接池
4. 统一响应结构
5. 统一错误处理
6. 健康检查接口
7. 路由注册入口
8. 请求日志中间件
9. CORS 配置
10. Docker 运行参数与配置文件对齐

## 8. Vue3 前端初始化步骤

## 8.1 创建项目

建议在 `frontend/` 下初始化：

1. 使用 `Vite` 创建 `Vue + TypeScript` 项目
2. 使用 `pnpm` 安装依赖

建议目录结构：

```text
frontend/
├─ index.html
├─ package.json
├─ tsconfig.json
├─ vite.config.ts
├─ .env.development
├─ .env.production
└─ src/
   ├─ main.ts
   ├─ App.vue
   ├─ api/
   ├─ assets/
   ├─ components/
   ├─ constants/
   ├─ layouts/
   ├─ router/
   ├─ stores/
   ├─ styles/
   ├─ types/
   ├─ utils/
   └─ views/
```

## 8.2 第一批依赖接入

建议先接入这些核心依赖：

1. `vue`
2. `vite`
3. `@vitejs/plugin-vue`
4. `typescript`
5. `vue-router`
6. `pinia`
7. `axios`
8. `element-plus`
9. `@element-plus/icons-vue`
10. `vue-tsc`
11. `eslint`
12. `@vue/eslint-config-typescript`
13. `prettier`
14. `vitest`
15. `@vueuse/core`

## 8.3 前端初始化最小闭环

第一阶段只要求前端达到以下结果：

1. 页面可以启动
2. 可以访问基础首页
3. Element Plus 已可用
4. Router 已可用
5. Pinia 已可用
6. Axios 请求封装已建立
7. 环境变量可读取

## 8.4 前端第一批基础模块

建议先搭这些基础能力：

1. 基础布局 `Layout`
2. 登录页骨架
3. 首页骨架
4. 路由配置
5. 状态管理入口
6. 请求拦截器
7. Token 存储工具
8. 全局错误提示
9. 通用表格与表单目录结构
10. Docker 构建输出与 `nginx` 代理对齐

## 9. 前后端联调准备步骤

在正式做业务模块前，建议先完成以下联调能力：

1. 后端提供 `/api/health`
2. 后端提供 `/api/system/profile` 模拟接口
3. 前端配置代理转发到后端
4. 前端首页成功请求后端健康检查接口
5. 前后端统一 API 前缀
6. 前后端统一时区与时间格式规范
7. `docker compose up --build` 可启动基础容器

## 10. 工程规范初始化步骤

建议在第一天就完成这些规范：

1. 后端日志级别与格式规范
2. 前端 ESLint + Prettier
3. `.editorconfig`
4. `.gitignore`
5. 环境变量模板 `.env.example`
6. README 启动说明
7. SQL 执行说明
8. Docker 启动说明

## 11. 第一阶段不要急着做的事

以下内容建议在项目最初两三天内先不要展开：

1. 不要一开始就引入 Redis
2. 不要一开始就做多租户
3. 不要一开始就做复杂数据权限
4. 不要一开始就封装过重的前端组件库
5. 不要一开始就引入第二套 ORM
6. 不要一开始就设计过度复杂的微服务拆分

原因：

1. 第一阶段目标是打通最小闭环
2. 先确保认证、用户、角色、菜单、权限链路成立
3. 复杂能力后续再分阶段补强

## 12. 初始化完成验收标准

当以下条件都满足时，可以认为项目初始化完成：

1. 后端项目可以启动
2. 前端项目可以启动
3. PostgreSQL 已完成建表与种子数据导入
4. 前端可以访问后端健康检查接口
5. 项目目录结构清晰
6. 核心依赖已安装完成
7. README 中具备基础启动说明
8. 后续可以直接开始做登录模块
9. `docker compose config` 检查通过

## 13. 推荐执行清单

如果按最务实的方式推进，建议直接照这个顺序执行：

1. 建立 `backend/`、`frontend/`、`docs/`、`sql/`、`docker/`、`scripts/`
2. 归档现有文档和 SQL 文件
3. 创建 PostgreSQL 数据库
4. 执行 `schema-postgresql.sql`
5. 执行 `seed-data.sql`
6. 初始化 Rust 后端工程
7. 写入后端第一批依赖
8. 跑通后端健康检查与数据库连接
9. 初始化 Vue3 前端工程
10. 写入前端第一批依赖
11. 跑通前端首页与接口请求
12. 配置前后端联调
13. 对齐 Dockerfile 与 `docker-compose.yml`
14. 然后进入登录、RBAC、菜单、用户模块开发

## 14. 下一步建议

这个清单确认后，最自然的下一步就是：

1. 按清单实际初始化项目目录
2. 以 `lhui-dev/rct` 为基础创建 Rust 后端骨架
3. 创建 Vue3 前端骨架
4. 把现有文档与 SQL 文件归档到规范目录中

一句话总结：

`先把基础工程和最小联调闭环搭起来，再开始做 RBAC 业务模块。`
