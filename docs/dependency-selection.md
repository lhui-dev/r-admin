# Rust + PostgreSQL + Vue3 管理系统依赖选型与版本对比

> 适用场景：基于 `Rust + PostgreSQL + Vue3` 构建 RBAC 管理系统，在正式初始化项目之前先确定前后端依赖组合。
>
> 版本核对日期：`2026-05-28`

## 1. 选型目标

这份文档关注的不是“能不能跑起来”，而是以下几个问题：

1. 哪些依赖适合作为项目初始化时的第一批核心依赖。
2. 哪些依赖虽然流行，但不适合在第一阶段就引入。
3. 哪些库存在多个可选方案，应该怎么取舍。
4. 版本应该追最新，还是优先稳定兼容。

本项目的总体建议是：

1. 后端优先选择 `Axum + Tokio + SQLx`。
2. 前端优先选择 `Vue 3 + Vite + TypeScript + Pinia + Vue Router + Element Plus`。
3. 第一阶段尽量少引入“重抽象”依赖，先把认证、RBAC、菜单、用户、角色、日志链路打通。

## 2. 总体推荐组合

如果现在就开始初始化项目，我建议直接采用下面这组：

### 2.1 后端推荐组合

1. `axum = "0.8.9"`
2. `tokio = { version = "~1.51", features = ["macros", "rt-multi-thread", "signal"] }`
3. `sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "time", "json", "migrate"] }`
4. `serde = { version = "1", features = ["derive"] }`
5. `serde_json = "1"`
6. `tower-http = { version = "0.6.11", features = ["cors", "trace", "compression-gzip"] }`
7. `tracing = "0.1.44"`
8. `tracing-subscriber = { version = "0.3.23", features = ["env-filter", "json"] }`
9. `thiserror = "2.0.18"`
10. `anyhow = "1.0.102"`
11. `uuid = { version = "1.23.1", features = ["v4", "serde"] }`
12. `config = "0.15.23"`
13. `validator = { version = "0.20.0", features = ["derive"] }`
14. `argon2 = "0.5.3"`
15. `jsonwebtoken = "10.4.0"`
16. `utoipa = "5.5.0"`

### 2.2 前端推荐组合

1. `vue = "3.5.16"`
2. `vite = "7.1.5"`
3. `@vitejs/plugin-vue = "6.0.1"`
4. `typescript = "~5.8.3"`
5. `vue-router = "4.5.1"`
6. `pinia = "3.0.3"`
7. `axios = "1.11.0"`
8. `element-plus = "2.11.2"`
9. `@element-plus/icons-vue = "2.3.2"`
10. `vue-tsc = "3.0.6"`
11. `eslint = "9.35.0"`
12. `@vue/eslint-config-typescript = "14.6.0"`
13. `prettier = "3.6.2"`
14. `vitest = "3.2.4"`
15. `@vueuse/core = "13.9.0"`

## 3. 后端依赖选型对比

## 3.1 Web 框架

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| Axum | `0.8.9` | 与 `tower` 生态结合紧密，类型安全，路由/中间件设计现代，适合 API 服务 | 对 Rust 新手略有门槛；`0.8.x` 需要较新 Rust | `推荐` |
| Actix Web | `4.13.0` | 性能强，生态成熟，历史案例多 | 风格与 `tower` 生态不完全一致；团队长期维护体验通常不如 Axum 统一 | `备选` |
| Poem | `3.1.12` | 上手快，OpenAPI 集成不错 | 团队与社区体量相对小一些 | `备选，不作为首选` |

建议：

1. 如果目标是“企业后台 API + 长期维护”，优先 `Axum`。
2. 如果团队已经深度使用 `Actix Web`，可以继续沿用，但新项目不必特意切过去。

## 3.2 异步运行时

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| Tokio 最新线 | `1.52.3` | 最新特性最全 | 版本推进较快 | `可用` |
| Tokio LTS 线 | `1.51.x` | 官方明确列为 LTS，适合稳定项目 | 不是最新 minor | `推荐初始化使用` |

建议：

1. 初始化阶段优先 `~1.51`，因为 Tokio 官方页面明确列出 `1.51.x` 为 LTS 分支。
2. 后续如果我们确实需要 `1.52+` 的能力，再升级到最新 minor。

## 3.3 数据访问层

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| SQLx | `0.9.0` 最新，`0.8.6` 同主线稳定版可见 | 原生 SQL、类型清晰、适合 RBAC 和复杂查询，性能与可控性好 | 需要自己维护 SQL 和 repository 层 | `强烈推荐` |
| SeaORM | `1.1.20`，`2.0.0-rc` 已存在 | CRUD 开发速度快，实体生成方便 | 抽象层更厚，复杂 SQL 最终仍会回到 SQLx/原生 SQL | `适合后台 CRUD 多的团队` |
| Diesel | `2.3.9` | 编译期约束强，成熟 | 宏与类型系统更重，异步与心智成本对团队要求更高 | `不建议作为本项目首选` |

建议：

1. RBAC、菜单树、日志检索、授权联表这类场景，`SQLx` 比 ORM 更顺手。
2. 尽管 `SQLx` 当前最新可见版本是 `0.9.0`，但项目初始化我更建议用 `0.8.6`，原因是生态示例、周边库兼容和团队踩坑成本更低。
3. 如果后续你希望快速生成实体与后台 CRUD，再考虑叠加 `SeaORM`，但不建议一开始就同时上两套数据访问层。

## 3.4 配置、日志、错误处理

| 类别 | 方案 | 当前可见版本 | 建议 |
| --- | --- | --- | --- |
| 配置 | `config` | `0.15.23` | `推荐`，适合多环境配置 |
| 日志 | `tracing` | `0.1.44` | `推荐` |
| 日志订阅 | `tracing-subscriber` | `0.3.23` | `推荐` |
| 业务错误 | `thiserror` | `2.0.18` | `推荐` |
| 顶层错误 | `anyhow` | `1.0.102` | `推荐` |

建议：

1. 业务层错误类型用 `thiserror`。
2. 启动、脚本、工具性流程错误用 `anyhow`。
3. HTTP 日志、SQL 慢查询、链路跟踪统一走 `tracing`。

## 3.5 安全与认证

| 类别 | 方案 | 当前可见版本 | 建议 |
| --- | --- | --- | --- |
| 密码哈希 | `argon2` | `0.5.3` | `推荐` |
| 旧方案 | `rust-argon2` | `3.0.0` | `不推荐新项目优先使用` |
| JWT | `jsonwebtoken` | `10.4.0` | `推荐` |
| 参数校验 | `validator` | `0.20.0` | `推荐` |

建议：

1. 新项目优先使用 RustCrypto 的 `argon2 0.5.3`，不要优先选老接口风格的 `rust-argon2 3.0.0`。
2. JWT 可以先用 `jsonwebtoken`，后续如果要上更强的会话撤销、设备管理、refresh token rotation，再补 Redis。

## 3.6 API 文档

| 方案 | 当前可见版本 | 优点 | 建议 |
| --- | --- | --- | --- |
| `utoipa` | `5.5.0` | Rust 生态里比较顺手的 OpenAPI 方案 | `推荐` |

建议：

1. 管理系统项目很适合从第一天就生成 OpenAPI。
2. 前端联调、权限接口清单、后续自动化测试都会更轻松。

## 4. 前端依赖选型对比

## 4.1 核心框架与构建工具

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| Vue | `3.5.16` | 生态成熟，适合中后台 | 无明显问题 | `推荐` |
| Vite | `7.1.5` | 启动快、构建快、Vue 配套成熟 | 需要较新的 Node.js | `推荐` |
| `@vitejs/plugin-vue` | `6.0.1` | Vue 官方构建链标准搭配 | 无明显问题 | `推荐` |

建议：

1. Vite 7 官方发布说明要求 Node.js `20.19+` 或 `22.12+`。
2. 结合 Node.js 官方发布页，当前更适合选 `Node.js 24 LTS` 或 `Node.js 22 LTS`，不建议再用已经 EOL 的 `Node.js 20`。

## 4.2 路由与状态管理

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| Vue Router | `4.5.1` | Vue3 标准路由方案 | 无明显问题 | `推荐` |
| Pinia | `3.0.3` | 轻量、类型友好、符合 Vue3 心智 | 无明显问题 | `推荐` |
| `@tanstack/vue-query` | `5.85.6` | 远程数据缓存、失效、重试做得很好 | 会引入第二套“服务端状态”心智模型 | `先不默认引入` |

建议：

1. 管理系统初始化阶段，`Pinia + Axios` 就够了。
2. 如果后面出现大量分页缓存、条件缓存、数据预取需求，再补 `@tanstack/vue-query`。

## 4.3 UI 组件库

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| Element Plus | `2.11.2` | 国内后台场景最常见，表单/表格/树/弹窗生态完整 | 包体积相对不小 | `首选` |
| Naive UI | `2.42.0` | 主题化灵活，TS 体验好 | 国内现成后台模板与团队经验通常少一些 | `备选` |
| Ant Design Vue | `4.2.6` | 企业风格成熟 | Vue 生态里整体惯用程度通常不如 Element Plus | `备选` |
| Vuetify | `3.9.7` | Material 体系完整 | 风格更重，企业后台不一定贴合 | `不建议首选` |

建议：

1. 既然目标是 RBAC 管理系统，首选 `Element Plus` 最稳。
2. 如果你特别在意主题系统和组件 API 一致性，`Naive UI` 是不错的第二选择。

## 4.4 请求、工具、工程化

| 类别 | 方案 | 当前可见版本 | 建议 |
| --- | --- | --- | --- |
| HTTP 客户端 | `axios` | `1.11.0` | `推荐` |
| Vue 工具集 | `@vueuse/core` | `13.9.0` | `推荐` |
| 类型检查 | `vue-tsc` | `3.0.6` | `推荐` |
| 测试 | `vitest` | `3.2.4` | `推荐` |
| ESLint | `eslint` | `9.35.0` | `推荐` |
| Vue TS ESLint 配置 | `@vue/eslint-config-typescript` | `14.6.0` | `推荐` |
| 格式化 | `prettier` | `3.6.2` | `推荐` |

建议：

1. 前端请求层先用 `axios`，自己封装 `request.ts`、拦截器、统一错误处理。
2. `@vueuse/core` 很适合做本地缓存、节流、防抖、窗口状态、深浅主题等通用能力。

## 4.5 自动导入插件

| 方案 | 当前可见版本 | 优点 | 风险/代价 | 建议 |
| --- | --- | --- | --- | --- |
| `unplugin-auto-import` | `20.1.0` | 减少样板代码 | 对新同事阅读不够直观 | `可选` |
| `unplugin-vue-components` | `29.0.0` | 自动按需注册组件 | 生成式导入会让边界变隐式 | `可选` |

建议：

1. 如果团队成员都熟悉 Vue3 工程化，这两个插件可以提高效率。
2. 如果项目更看重“显式依赖、易读、易审查”，初始化阶段可以先不上。

## 5. 初始化阶段不建议一开始就上的依赖

以下依赖不是不能用，而是不建议在第一阶段就引入：

1. `SeaORM`
原因：会和 `SQLx` 的边界冲突，容易一开始就出现两套数据访问风格。

2. `@tanstack/vue-query`
原因：RBAC 管理系统前期数据流相对简单，先用 `Pinia + Axios` 更直接。

3. Redis 客户端
原因：如果第一阶段只是登录、菜单、角色权限，不一定需要立刻引入缓存与 token 黑名单。

4. 消息队列客户端
原因：RBAC 底座项目第一阶段通常没有异步业务编排刚需。

5. 复杂表格二次封装库
原因：最好先基于 `Element Plus` 原生表格把业务抽象摸清楚，再决定是否上增强表格方案。

## 6. 初始化建议版本策略

## 6.1 Rust 侧

1. `axum` 建议直接锁定到 `0.8.9`。
2. `tokio` 建议使用 `~1.51`，优先走 LTS minor。
3. `sqlx` 建议先用 `0.8.6`，不要在项目第一天就上 `0.9.0`。
4. `serde`、`serde_json`、`uuid` 可以正常跟随当前稳定线。
5. `argon2` 直接用 `0.5.3`。

## 6.2 前端侧

1. `vite` 建议锁定到 `7.1.x` 当前稳定线。
2. `typescript` 建议锁定 minor，如 `~5.8.3`。
3. `element-plus` 可以先用当前稳定版 `2.11.2`。
4. `eslint`、`prettier`、`vue-tsc` 跟随当前稳定线即可。

## 7. 推荐初始化清单

如果接下来准备正式初始化项目，我建议第一批只加这些：

### 7.1 Rust 第一批

```toml
[package]
edition = "2024"
rust-version = "1.80"

[dependencies]
axum = "0.8.9"
tokio = { version = "~1.51", features = ["macros", "rt-multi-thread", "signal"] }
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "time", "json", "migrate"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6.11", features = ["cors", "trace", "compression-gzip"] }
tracing = "0.1.44"
tracing-subscriber = { version = "0.3.23", features = ["env-filter", "json"] }
thiserror = "2.0.18"
anyhow = "1.0.102"
uuid = { version = "1.23.1", features = ["v4", "serde"] }
config = "0.15.23"
validator = { version = "0.20.0", features = ["derive"] }
argon2 = "0.5.3"
jsonwebtoken = "10.4.0"
utoipa = "5.5.0"
```

### 7.2 Vue 第一批

```json
{
  "dependencies": {
    "vue": "3.5.16",
    "vue-router": "4.5.1",
    "pinia": "3.0.3",
    "axios": "1.11.0",
    "element-plus": "2.11.2",
    "@element-plus/icons-vue": "2.3.2",
    "@vueuse/core": "13.9.0"
  },
  "devDependencies": {
    "vite": "7.1.5",
    "@vitejs/plugin-vue": "6.0.1",
    "typescript": "~5.8.3",
    "vue-tsc": "3.0.6",
    "eslint": "9.35.0",
    "@vue/eslint-config-typescript": "14.6.0",
    "prettier": "3.6.2",
    "vitest": "3.2.4"
  }
}
```

## 8. 最终建议

如果以“稳、好维护、适合长期演进”的标准来定，这个项目的依赖选型建议可以直接定为：

1. 后端：`Axum + Tokio(LTS minor) + SQLx + tracing + thiserror + argon2 + jsonwebtoken + utoipa`
2. 前端：`Vue3 + Vite + TypeScript + Vue Router + Pinia + Axios + Element Plus`
3. 初始化阶段先不引入 `SeaORM`、`Vue Query`、Redis、MQ 这类增强依赖

一句话版本：

`先用轻抽象打稳底座，再按业务复杂度逐步加能力。`

## 9. 参考来源

以下版本信息已按 `2026-05-28` 前后可见的官方/主站页面核对：

1. Axum Docs.rs: <https://docs.rs/crate/axum/latest>
2. Tokio Docs.rs: <https://docs.rs/crate/tokio/latest>
3. SQLx Docs.rs: <https://docs.rs/crate/sqlx/latest>
4. SeaORM Docs.rs: <https://docs.rs/crate/sea-orm/latest>
5. Diesel Docs.rs: <https://docs.rs/crate/diesel/latest>
6. Tower HTTP Docs.rs: <https://docs.rs/crate/tower-http/latest>
7. Tracing Docs.rs: <https://docs.rs/crate/tracing/latest>
8. Tracing Subscriber Docs.rs: <https://docs.rs/tracing-subscriber/latest/index.html>
9. Serde Docs.rs: <https://docs.rs/crate/serde/latest>
10. Anyhow Docs.rs: <https://docs.rs/crate/anyhow/latest>
11. Thiserror Docs.rs: <https://docs.rs/thiserror>
12. Uuid Docs.rs: <https://docs.rs/crate/uuid/latest>
13. Validator Docs.rs: <https://docs.rs/crate/validator/latest>
14. Config Docs.rs: <https://docs.rs/crate/config/latest>
15. Jsonwebtoken Docs.rs: <https://docs.rs/crate/jsonwebtoken/latest>
16. Argon2 Docs.rs: <https://docs.rs/argon2>
17. Rust Argon2 Docs.rs: <https://docs.rs/rust-argon2/latest/argon2/>
18. Utoipa Docs.rs: <https://docs.rs/crate/utoipa/latest>
19. Vue npm: <https://www.npmjs.com/package/vue>
20. Vite npm: <https://www.npmjs.com/package/vite>
21. Vite Releases: <https://vite.dev/releases>
22. Vite 7 发布说明: <https://vite.dev/blog/announcing-vite7>
23. TypeScript npm: <https://www.npmjs.com/package/typescript>
24. Vue Router npm: <https://www.npmjs.com/package/vue-router>
25. Pinia npm: <https://www.npmjs.com/package/pinia>
26. Element Plus npm: <https://www.npmjs.com/package/element-plus>
27. Naive UI npm: <https://www.npmjs.com/package/naive-ui>
28. Ant Design Vue npm: <https://www.npmjs.com/package/ant-design-vue>
29. Vuetify npm: <https://www.npmjs.com/package/vuetify>
30. Axios npm: <https://www.npmjs.com/package/axios>
31. Vue TSC npm: <https://www.npmjs.com/package/vue-tsc>
32. ESLint npm: <https://www.npmjs.com/package/eslint>
33. Vue ESLint TS Config npm: <https://www.npmjs.com/package/@vue/eslint-config-typescript>
34. Prettier npm: <https://www.npmjs.com/package/prettier>
35. Vitest npm: <https://www.npmjs.com/package/vitest>
36. VueUse npm: <https://www.npmjs.com/package/@vueuse/core>
37. TanStack Vue Query npm: <https://www.npmjs.com/package/@tanstack/vue-query>
38. Unplugin Auto Import npm: <https://www.npmjs.com/package/unplugin-auto-import>
39. Unplugin Vue Components npm: <https://www.npmjs.com/package/unplugin-vue-components>
40. Node.js Releases: <https://nodejs.org/tr/download/releases>
41. Node.js EOL: <https://nodejs.org/en/eol>
