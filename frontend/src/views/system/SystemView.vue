<template>
  <section class="system-view">
    <el-card
      class="system-view__card"
      shadow="never"
    >
      <template #header>
        <div class="system-view__header">
          <div>
            <strong>系统配置中心</strong>
            <p>当前页用于集中管理租户配置、安全策略、审计规则与后台基础参数。</p>
          </div>
          <el-tag round type="primary">基础配置</el-tag>
        </div>
      </template>

      <div class="system-view__content">
        <article>
          <span>租户配置</span>
          <h2>默认租户 / tenant_default</h2>
          <p>支持配置租户标识、数据隔离范围、启停状态与管理员归属。</p>
        </article>

        <article>
          <span>安全策略</span>
          <h2>密码有效期 90 天</h2>
          <p>可维护登录失败阈值、会话时长、密码复杂度与二次验证策略。</p>
        </article>

        <article>
          <span>审计策略</span>
          <h2>日志保留 180 天</h2>
          <p>支持记录登录、授权、菜单变更、权限分配与异常操作审计事件。</p>
        </article>

        <article>
          <span>导航配置</span>
          <h2>菜单树 / 路由 / 按钮权限</h2>
          <p>用于统一维护后台菜单显示、路由映射、按钮显隐与权限编码绑定。</p>
        </article>
      </div>
    </el-card>
  </section>
</template>

<style scoped>
.system-view__card {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-xl);
  background: var(--app-surface);
  box-shadow: var(--app-shadow);
}

:deep(.system-view__card .el-card__header) {
  border-bottom-color: var(--app-border);
}

.system-view__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.system-view__header strong {
  display: block;
  font-size: 18px;
}

.system-view__header p {
  margin: 6px 0 0;
  color: var(--app-text-soft);
}

.system-view__content {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
}

.system-view__content article {
  padding: 20px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-soft);
}

.system-view__content span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.system-view__content h2 {
  margin: 10px 0 8px;
  font-size: 20px;
}

.system-view__content p {
  margin: 0;
  color: var(--app-text-soft);
}

@media (max-width: 640px) {
  .system-view__header {
    align-items: flex-start;
    flex-direction: column;
  }
}
</style>
