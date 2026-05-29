# CI 工作流说明

## 1. 目标

本文件用于说明当前仓库 GitHub Actions 的职责范围、触发方式与后续扩展方向，避免 CI 只存在于 workflow 文件中而缺少文档说明。

## 2. 当前工作流

当前仓库包含以下 CI：

1. [后端 CI](../.github/workflows/backend-ci.yml)
2. [前端 CI](../.github/workflows/frontend-ci.yml)

## 3. 后端 CI

工作流文件：

1. [backend-ci.yml](../.github/workflows/backend-ci.yml)

当前检查项：

1. `cargo check`

当前定位：

1. 验证 Rust 后端代码可通过基础编译检查
2. 作为后端最小质量门，先保证主分支合并前有基本静态校验

## 4. 前端 CI

工作流文件：

1. [frontend-ci.yml](../.github/workflows/frontend-ci.yml)

当前检查项：

1. `npm ci`
2. `npm run typecheck`
3. `npm run build`

当前定位：

1. 验证前端依赖安装是否正常
2. 验证 TypeScript 类型检查是否通过
3. 验证生产构建是否可完成

## 5. 触发规则

当前前后端 CI 都在以下场景触发：

1. `push` 到 `main`
2. `push` 到 `feature/**`
3. `push` 到 `fix/**`
4. `push` 到 `chore/**`
5. `push` 到 `docs/**`
6. `pull_request` 到 `main`

## 6. 当前策略

当前策略是：

1. 先保证前后端各自有最小 CI
2. 先让 workflow 稳定执行
3. 再逐步纳入 `main` 分支保护

## 7. 后续建议

后续可以按下面顺序增强：

1. 后端补充 `cargo test`
2. 前端补充 `lint` 与单元测试
3. 增加 Docker 构建检查
4. 将关键检查设置为 `main` 分支 required checks

一句话总结：

`先建立最小可用 CI，再逐步增强质量门。`
