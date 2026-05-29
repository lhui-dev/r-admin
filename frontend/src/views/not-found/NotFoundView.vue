<template>
  <section class="not-found-view">
    <el-result
      icon="warning"
      title="页面不存在"
      sub-title="当前路由还没有配置对应页面。"
    >
      <template #extra>
        <RouterLink to="/dashboard">
          <el-button type="primary">返回工作台</el-button>
        </RouterLink>
      </template>
    </el-result>
  </section>
</template>

<style scoped>
.not-found-view {
  display: grid;
  min-height: 60vh;
  place-items: center;
}
</style>
