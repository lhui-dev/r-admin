# r-admin

基于 `Rust + PostgreSQL + Vue3` 的 RBAC 管理系统基础工程。

当前仓库处于项目初始化阶段，已经完成需求分析、数据库设计、E-R 图、依赖选型、种子数据和 Docker 容器化骨架整理，接下来将基于 `lhui-dev/rct` 作为 Rust 后端模板继续落地。

## 1. 技术方向

后端：

1. Rust
2. PostgreSQL
3. Axum
4. SQLx
5. JWT

前端：

1. Vue 3
2. Vite
3. TypeScript
4. Pinia
5. Vue Router
6. Element Plus

## 2. 当前目录结构

```text
r-admin/
├─ backend/                  # Rust 后端工程目录（待按 lhui-dev/rct 初始化）
├─ frontend/                 # Vue3 前端工程目录（待初始化）
├─ docs/                     # 项目文档
├─ sql/                      # 建表与种子数据 SQL
├─ docker/                   # Dockerfile 与 Nginx 配置
├─ scripts/                  # 辅助脚本目录
├─ docker-compose.yml        # 项目主容器编排文件
├─ docker-compose.db-admin.yaml
├─ .dockerignore
├─ .editorconfig
├─ .env.docker.example
├─ .gitignore
└─ README.md
```

## 3. 关键文档

文档总入口：

1. [docs/README.md](docs/README.md)

核心文档：

1. [需求分析](docs/rbac-system-requirements.md)
2. [依赖选型](docs/dependency-selection.md)
3. [项目初始化清单](docs/project-init-checklist.md)
4. [容器化说明](docs/dockerization.md)
5. [后端初始化计划](docs/backend-init-plan.md)
6. [E-R 图 SVG](docs/rbac-er-diagram.svg)
7. [E-R 图 Mermaid](docs/rbac-er-diagram.mmd)

## 4. 数据库脚本

1. [建表脚本](sql/schema-postgresql.sql)
2. [种子数据](sql/seed-data.sql)

说明：

1. 种子数据中的 `admin` 密码哈希目前仍是占位值
2. 真正接入登录前，需要按 Rust 后端实际密码算法替换

## 5. Docker 入口

主编排文件：

1. [docker-compose.yml](docker-compose.yml)

数据库管理辅助编排：

1. [docker-compose.pg-admin.yaml](docker/docker-compose.pg-admin.yaml)

说明：

1. `docker-compose.yml` 用于项目主环境：`postgres + backend + frontend`
2. `docker-compose.db-admin.yaml` 用于数据库管理辅助场景

## 6. 当前进度

已经完成：

1. 需求分析
2. 数据库表设计
3. E-R 图绘制
4. PostgreSQL 建表 SQL
5. 初始化种子数据 SQL
6. 依赖选型与版本建议
7. Docker 容器化骨架
8. Git 仓库初始化与基础忽略规则
9. Rust 后端最小骨架初始化

待完成：

1. 初始化 `frontend/`
2. 打通前后端最小运行闭环
3. 接入认证、用户、角色、菜单、权限模块

## 7. 推荐初始化顺序

1. 按 `lhui-dev/rct` 初始化 [backend](backend)
2. 初始化 [frontend](frontend)
3. 对齐 Dockerfile 与运行参数
4. 打通 `/api/health`
5. 接入登录与 RBAC 主链路

## 8. 备注

这个仓库当前适合作为“项目基础骨架初始化”的第一次提交。
