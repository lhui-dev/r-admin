# backend

`r-admin` 的 Rust 后端工程。

当前阶段已完成：

1. `Axum` 最小服务骨架
2. `SQLx` PostgreSQL 连接池
3. 配置加载
4. 统一响应结构
5. `/api/health` 健康检查接口
6. 基础日志与 CORS/Trace 中间件

## 启动前准备

1. 复制 `.env.example` 为 `.env`
2. 确认 PostgreSQL 已启动
3. 确认 `DATABASE_URL` 可连接

## 本地运行

```bash
cargo run
```

默认健康检查：

```text
GET /api/health
```
