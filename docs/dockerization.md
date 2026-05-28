# Docker 容器化说明

## 1. 当前产物

已新增以下容器化基础文件：

1. [docker-compose.yml](../docker-compose.yml)
2. [.dockerignore](../.dockerignore)
3. [.env.docker.example](../.env.docker.example)
4. [docker/backend.Dockerfile](../docker/backend.Dockerfile)
5. [docker/frontend.Dockerfile](../docker/frontend.Dockerfile)
6. [docker/nginx/default.conf](../docker/nginx/default.conf)

## 2. 当前容器化方案

采用三容器结构：

1. `postgres`
2. `backend`
3. `frontend`

职责划分：

1. `postgres` 负责数据库与初始化 SQL 执行
2. `backend` 负责 Rust API 服务
3. `frontend` 负责构建 Vue3 静态资源，并通过 `nginx` 提供访问

## 3. 使用方式

推荐流程：

1. 复制 [.env.docker.example](../.env.docker.example) 为 `.env`
2. 确认 `backend/` 和 `frontend/` 项目已初始化
3. 根据 Rust 模板实际二进制名修改 `BACKEND_APP_NAME`
4. 执行 `docker compose up --build`

## 4. 需要注意的占位项

当前文件是“可落地骨架”，但还需要在真正初始化项目后做两处对齐：

1. `BACKEND_APP_NAME`
说明：
默认值是 `backend`，但如果 `lhui-dev/rct` 模板生成的 Rust 可执行文件名不同，需要在 `.env` 中修改它。

2. 后端配置文件路径
说明：
如果 `lhui-dev/rct` 模板有自己的配置目录、启动参数或运行时资源目录，后续需要把 [docker/backend.Dockerfile](../docker/backend.Dockerfile) 再对齐一次。

## 5. 与当前 SQL 的关系

`docker-compose.yml` 当前直接把 `sql/` 目录下的两份 SQL 文件挂载到了 PostgreSQL 的 `/docker-entrypoint-initdb.d`：

1. [schema-postgresql.sql](../sql/schema-postgresql.sql)
2. [seed-data.sql](../sql/seed-data.sql)

这意味着：

1. 当数据库卷首次创建时，会自动执行建表和种子脚本
2. 当前项目结构已经与 `docker-compose.yml` 对齐，不需要再额外调整 SQL 路径
3. 如果数据库数据卷已经存在，初始化脚本不会自动重复执行

## 6. 推荐目录状态

为了让容器化真正可用，建议把项目整理成如下结构：

```text
r-admin/
├─ backend/
├─ frontend/
├─ sql/
├─ docker/
├─ docker-compose.yml
└─ .env
```

## 7. 下一步建议

最合理的下一步是：

1. 先整理 `docs/`、`sql/` 目录
2. 再按 `lhui-dev/rct` 初始化 `backend/`
3. 再初始化 `frontend/`
4. 最后回过头把 Dockerfile 和 Compose 做一次模板对齐

一句话版本：

`容器化骨架已经准备好，等后端前端项目真正落地后，再做一次轻量对齐就能投入使用。`
