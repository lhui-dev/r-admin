# 前端初始化计划

## 1. 目标

本阶段目标是在 `feature/init-frontend` 分支上完成 Vue3 前端工程骨架初始化，为后续登录、动态菜单、RBAC 页面和前后端联调提供基础运行能力。

## 2. 技术路线

前端采用：

1. Vue 3
2. Vite
3. TypeScript
4. Vue Router
5. Pinia
6. Element Plus
7. Axios

## 3. 组件复用策略

前端基础能力参考：

- [lhui-dev/vue3-tools](https://github.com/lhui-dev/vue3-tools)

采用策略：

`选择性复用，而不是整仓照搬。`

优先复用的能力包括：

1. `layouts`
2. `base-ui`
3. `hooks`
4. `utils/request.ts`
5. 动态菜单映射思路

暂不直接搬运：

1. 示例业务页
2. 演示型组件
3. 与当前 RBAC 数据结构不一致的页面逻辑

## 4. 第一批落地内容

本阶段已优先完成：

1. `frontend/` Vite 工程骨架
2. `Vue Router + Pinia + Element Plus` 接入
3. 基础后台布局 `AppLayout`
4. 请求封装 `utils/request.ts`
5. 健康检查演示页 `DashboardView`
6. 与后端 `/api/health` 的联调入口
7. `base-ui` 目录占位

## 5. 下一步

后续建议按以下顺序继续扩展：

1. 引入登录页
2. 引入动态菜单
3. 迁移 `base-table` / `base-form` / `page-search`
4. 对接用户、角色、菜单管理页面
5. 补充前端 CI
