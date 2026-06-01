<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()

const featureKey = computed(() => String(route.params.feature ?? 'default'))
const featureContent = computed(() => {
  const contentMap: Record<string, { title: string, description: string, phase: string, nextStep: string }> = {
    users: {
      title: '用户管理',
      description: '这里将承接用户查询、状态维护、角色分配与密码重置等 RBAC 基础能力。',
      phase: '页面骨架待接入',
      nextStep: '优先补列表、搜索区和基础操作栏。',
    },
    roles: {
      title: '角色管理',
      description: '这里将维护角色定义、角色状态和角色授权范围，是 RBAC 的核心配置入口之一。',
      phase: '业务模型已规划',
      nextStep: '下一步将接入角色列表与权限绑定入口。',
    },
    permissions: {
      title: '权限点管理',
      description: '这里会承接页面权限、按钮权限和接口权限的统一可视化维护。',
      phase: '权限结构待接入',
      nextStep: '后续将与菜单和角色授权能力一起推进。',
    },
    menus: {
      title: '菜单管理',
      description: '这里将维护菜单树、图标、排序和可见状态，并与角色授权联动。',
      phase: '动态菜单初始化中',
      nextStep: '当前优先完成菜单接口与前端初始化链路。',
    },
    departments: {
      title: '部门管理',
      description: '这里会维护组织结构、部门负责人和用户归属关系，为数据权限打基础。',
      phase: '组织模块待接入',
      nextStep: '后续与用户管理和岗位能力联动推进。',
    },
    'audit-logs': {
      title: '审计日志',
      description: '这里会集中展示登录日志、操作日志和关键行为追踪记录。',
      phase: '日志展示待接入',
      nextStep: '后续优先承接分页列表和筛选条件。',
    },
    'permission-center': {
      title: '权限中心',
      description: '这里将串起角色、菜单、权限点和授权关系，成为 RBAC 配置总入口。',
      phase: '主链路基础已完成',
      nextStep: '先补齐菜单与页面骨架，再进入细分模块。',
    },
    organization: {
      title: '组织架构',
      description: '这里会承接部门、岗位、成员归属和组织层级的统一视图。',
      phase: '组织模块规划中',
      nextStep: '待用户与部门基础页面形成后再扩展。',
    },
    'audit-center': {
      title: '审计中心',
      description: '这里将集中承接日志查询、异常追踪和关键配置变更审计。',
      phase: '审计能力规划中',
      nextStep: '后续会与登录日志和操作日志模块打通。',
    },
    default: {
      title: '功能建设中',
      description: '当前入口已纳入导航结构，但对应业务页面仍在分阶段接入。',
      phase: '占位页已接入',
      nextStep: '后续会根据迭代优先级逐步替换为真实页面。',
    },
  }

  return contentMap[featureKey.value] ?? contentMap.default
})
</script>

<template>
  <section class="construction-view">
    <el-card
      class="construction-view__hero"
      shadow="never"
    >
      <div class="construction-view__hero-copy">
        <span class="construction-view__eyebrow">功能占位页</span>
        <h1>{{ featureContent.title }}</h1>
        <p>{{ featureContent.description }}</p>
      </div>

      <div class="construction-view__hero-actions">
        <button
          type="button"
          class="construction-view__button construction-view__button--primary"
          @click="router.push('/dashboard')"
        >
          返回概览看板
        </button>
        <button
          type="button"
          class="construction-view__button"
          @click="router.back()"
        >
          返回上一页
        </button>
      </div>
    </el-card>

    <div class="construction-view__grid">
      <el-card
        class="construction-view__card"
        shadow="never"
      >
        <template #header>
          <div class="construction-view__card-header">
            <strong>当前阶段</strong>
            <span>模块建设状态</span>
          </div>
        </template>

        <div class="construction-view__block">
          <article class="construction-view__item">
            <span>状态</span>
            <strong>{{ featureContent.phase }}</strong>
          </article>
          <article class="construction-view__item">
            <span>下一步</span>
            <strong>{{ featureContent.nextStep }}</strong>
          </article>
        </div>
      </el-card>

      <el-card
        class="construction-view__card"
        shadow="never"
      >
        <template #header>
          <div class="construction-view__card-header">
            <strong>当前建议</strong>
            <span>进入真实页面前</span>
          </div>
        </template>

        <div class="construction-view__checklist">
          <article>先稳定菜单、认证和路由主链路，再进入业务 CRUD 页面开发。</article>
          <article>优先确定接口契约、搜索条件和列表字段，避免页面返工。</article>
          <article>后续真实页面接入后，这个占位页可以平滑移除，不影响导航结构。</article>
        </div>
      </el-card>
    </div>
  </section>
</template>

<style scoped>
.construction-view {
  display: grid;
  gap: 14px;
}

.construction-view__hero,
.construction-view__card {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  box-shadow: var(--app-shadow);
}

.construction-view__hero {
  display: grid;
  gap: 18px;
}

.construction-view__hero-copy {
  display: grid;
  gap: 8px;
}

.construction-view__eyebrow {
  color: var(--app-primary);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.08em;
}

.construction-view__hero-copy h1 {
  margin: 0;
  color: #111827;
  font-size: clamp(24px, 2.8vw, 32px);
  line-height: 1.2;
}

.construction-view__hero-copy p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 14px;
  line-height: 1.75;
  max-width: 760px;
}

.construction-view__hero-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.construction-view__button {
  height: 36px;
  padding: 0 16px;
  border: 1px solid var(--app-border);
  border-radius: 999px;
  background: #fff;
  color: #334155;
  font-size: 13px;
  font-weight: 700;
  cursor: pointer;
}

.construction-view__button--primary {
  border-color: transparent;
  background: var(--app-primary);
  color: #fff;
}

.construction-view__grid {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.construction-view__card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.construction-view__card-header strong {
  color: #111827;
  font-size: 16px;
}

.construction-view__card-header span,
.construction-view__item span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.construction-view__block,
.construction-view__checklist {
  display: grid;
  gap: 12px;
}

.construction-view__item,
.construction-view__checklist article {
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.construction-view__item {
  display: grid;
  gap: 6px;
}

.construction-view__item strong,
.construction-view__checklist article {
  color: #111827;
  font-size: 14px;
  line-height: 1.7;
}

:deep(.construction-view__card .el-card__header) {
  border-bottom-color: var(--app-border);
}

@media (max-width: 900px) {
  .construction-view__grid {
    grid-template-columns: 1fr;
  }
}
</style>
