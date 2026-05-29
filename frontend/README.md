# frontend

`r-admin` 的 Vue3 前端工程。

当前阶段已完成：

1. `Vite + Vue 3 + TypeScript` 工程骨架
2. `Vue Router + Pinia + Element Plus` 基础接入
3. 后台布局 `AppLayout`
4. Axios 请求封装
5. `/api/health` 联调页面
6. 与 `lhui-dev/vue3-tools` 对齐的基础目录结构

## 本地运行

```bash
npm install
npm run dev
```

默认开发地址：

```text
http://127.0.0.1:5173
```

## 本地代理说明

默认开发代理目标：

```text
http://127.0.0.1:8080
```

如果你本机 `8080` 已被占用，可以新增本地文件：

```text
.env.development.local
```

内容例如：

```text
VITE_PROXY_TARGET=http://127.0.0.1:18080
```
