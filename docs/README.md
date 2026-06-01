# 文档索引

这里汇总当前项目的设计文档、初始化清单与容器化说明，方便在线上仓库中快速浏览。

## 1. 核心文档

1. [需求分析与数据库设计](./rbac-system-requirements.md)
2. [依赖选型与版本建议](./dependency-selection.md)
3. [项目初始化清单](./project-init-checklist.md)
4. [Docker 容器化说明](./dockerization.md)
5. [后端初始化计划](./backend-init-plan.md)
6. [前端初始化计划](./frontend-init-plan.md)
7. [认证初始化计划](./auth-init-plan.md)
8. [认证初始化执行清单](./auth-init-checklist.md)
9. [前端认证接入 TODO 清单](./frontend-auth-todo.md)
10. [APIFox 认证调试说明](./apifox-auth-debug.md)
11. [APIFox 调试脚本清单](./apifox-debug-scripts.md)
12. [动态菜单初始化方案](./dynamic-menu-init-plan.md)
13. [菜单接口设计](./auth-menus-api-design.md)
14. [后端菜单实现说明](./backend-auth-menus-implementation-plan.md)
15. [CI 工作流说明](./ci-workflows.md)

## 2. 图与模型

1. [E-R 图 SVG](./rbac-er-diagram.svg)
2. [E-R 图 Mermaid 源码](./rbac-er-diagram.mmd)
3. [APIFox OpenAPI 导入文件](./apifox-auth.openapi.json)

## 3. 相关脚本入口

1. [建表脚本](../sql/schema-postgresql.sql)
2. [种子数据脚本](../sql/seed-data.sql)
3. [主容器编排](../docker-compose.yml)
4. [数据库管理辅助编排](../docker/docker-compose.pg-admin.yaml)
5. [后端 CI Workflow](../.github/workflows/backend-ci.yml)
6. [前端 CI Workflow](../.github/workflows/frontend-ci.yml)
7. [APIFox 脚本目录](./apifox-scripts/)

## 4. 推荐阅读顺序

1. 先看 [需求分析与数据库设计](./rbac-system-requirements.md)
2. 再看 [依赖选型与版本建议](./dependency-selection.md)
3. 然后看 [项目初始化清单](./project-init-checklist.md)
4. 再看 [后端初始化计划](./backend-init-plan.md)
5. 再看 [前端初始化计划](./frontend-init-plan.md)
6. 再看 [认证初始化计划](./auth-init-plan.md)
7. 再看 [认证初始化执行清单](./auth-init-checklist.md)
8. 再看 [前端认证接入 TODO 清单](./frontend-auth-todo.md)
9. 再看 [动态菜单初始化方案](./dynamic-menu-init-plan.md)
10. 再看 [菜单接口设计](./auth-menus-api-design.md)
11. 再看 [后端菜单实现说明](./backend-auth-menus-implementation-plan.md)
12. 再看 [APIFox 认证调试说明](./apifox-auth-debug.md)
13. 再看 [APIFox 调试脚本清单](./apifox-debug-scripts.md)
14. 再看 [CI 工作流说明](./ci-workflows.md)
15. 最后结合 [Docker 容器化说明](./dockerization.md) 落地环境
