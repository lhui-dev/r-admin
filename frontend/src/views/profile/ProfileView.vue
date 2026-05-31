<script setup lang="ts">
import { computed, ref } from 'vue'

import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const permissionsExpanded = ref(false)

const currentUser = computed(() => authStore.currentUser)
const roleCount = computed(() => authStore.roles.length)
const permissionCount = computed(() => authStore.permissions.length)
const permissionGroupCount = computed(() => permissionGroups.value.length)
const basicInfoItems = computed(() => [
  { label: '用户 ID', value: String(currentUser.value?.id ?? '--') },
  { label: '登录账号', value: currentUser.value?.username ?? '--' },
  { label: '昵称', value: currentUser.value?.nickname ?? '--' },
  { label: '真实姓名', value: currentUser.value?.real_name ?? '未设置' },
])
const identityInfoItems = computed(() => [
  {
    label: '账号类型',
    value: currentUser.value?.is_super_admin ? '超级管理员' : '普通 RBAC 成员',
  },
  {
    label: '登录状态',
    value: authStore.isAuthenticated ? '认证有效' : '待登录',
  },
  { label: '角色数量', value: `${roleCount.value} 个` },
  { label: '权限分组', value: `${permissionGroupCount.value} 组` },
])
const roleDisplayItems = computed(() => {
  return authStore.roles.map((roleCode) => ({
    code: roleCode,
    label: formatRoleLabel(roleCode),
  }))
})
const permissionGroups = computed(() => {
  // Group raw permission codes into display-friendly buckets so the page stays
  // readable even when /me returns dozens of fine-grained permission entries.
  const groups = new Map<string, { key: string, label: string, actions: string[], codes: string[] }>()

  for (const permissionCode of authStore.permissions) {
    const segments = permissionCode.split(':')
    const groupKey = resolvePermissionGroupKey(segments)
    const existingGroup = groups.get(groupKey)
    const action = resolvePermissionActionCode(segments)

    if (existingGroup) {
      existingGroup.actions.push(action)
      existingGroup.codes.push(permissionCode)
      continue
    }

    groups.set(groupKey, {
      key: groupKey,
      label: formatPermissionGroupLabel(groupKey),
      actions: [action],
      codes: [permissionCode],
    })
  }

  return Array.from(groups.values())
})
const visiblePermissionGroups = computed(() => {
  if (permissionsExpanded.value) {
    return permissionGroups.value
  }

  return permissionGroups.value.slice(0, 6)
})
const hiddenPermissionGroupCount = computed(() => {
  return Math.max(permissionGroups.value.length - visiblePermissionGroups.value.length, 0)
})

function formatRoleLabel(roleCode: string) {
  const roleMap: Record<string, string> = {
    super_admin: '超级管理员',
    system_admin: '系统管理员',
    auditor: '审计员',
  }

  return roleMap[roleCode] ?? humanizeCode(roleCode)
}

function formatPermissionGroupLabel(groupKey: string) {
  const groupMap: Record<string, string> = {
    dashboard: '控制台看板',
    'system:user': '用户管理',
    'system:role': '角色管理',
    'system:menu': '菜单管理',
    'system:dept': '部门管理',
    'system:dict': '字典管理',
    'system:post': '岗位管理',
    'system:config': '系统配置',
    'system:log:login': '登录日志',
    'system:log:operation': '操作日志',
  }

  return groupMap[groupKey] ?? humanizeCode(groupKey)
}

function formatPermissionActionLabel(actionCode: string) {
  const actionMap: Record<string, string> = {
    view: '查看',
    list: '列表',
    create: '新增',
    update: '更新',
    delete: '删除',
    'assign-role': '分配角色',
    'assign-permission': '分配权限',
    'reset-password': '重置密码',
  }

  return actionMap[actionCode] ?? humanizeCode(actionCode)
}

function humanizeCode(code: string) {
  return code
    .split(/[:_-]/g)
    .filter(Boolean)
    .map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
    .join(' / ')
}

function resolvePermissionGroupKey(segments: string[]) {
  if (segments.length <= 1) {
    return segments[0] ?? 'unknown'
  }

  if (segments.length === 2) {
    return segments[0]
  }

  if (segments.length >= 4) {
    return `${segments[0]}:${segments[1]}:${segments[2]}`
  }

  return `${segments[0]}:${segments[1]}`
}

function resolvePermissionActionCode(segments: string[]) {
  if (segments.length <= 1) {
    return segments[0] ?? 'unknown'
  }

  return segments[segments.length - 1]
}
</script>

<template>
  <section class="profile-view">
    <el-card
      class="profile-view__hero"
      shadow="never"
    >
      <div class="profile-view__hero-main">
        <div class="profile-view__avatar">
          {{ authStore.avatarText }}
        </div>

        <div class="profile-view__hero-copy">
          <div class="profile-view__hero-heading">
            <h1>{{ authStore.displayName }}</h1>
            <el-tag
              v-if="currentUser?.is_super_admin"
              round
              type="danger"
            >
              超级管理员
            </el-tag>
            <el-tag
              v-else
              round
              type="primary"
            >
              RBAC 成员
            </el-tag>
          </div>

          <div class="profile-view__hero-subline">
            <span>@{{ currentUser?.username ?? '--' }}</span>
            <span>账号 ID：{{ currentUser?.id ?? '--' }}</span>
            <span>{{ currentUser?.real_name ?? '未完善实名信息' }}</span>
          </div>

          <p>
            当前页面用于展示账号基础信息、角色归属与权限概览，后续可在这里继续扩展安全设置与个人资料编辑。
          </p>
        </div>
      </div>
    </el-card>

    <div class="profile-view__grid">
      <el-card
        class="profile-view__card"
        shadow="never"
      >
        <template #header>
          <div class="profile-view__card-header">
            <strong>基础资料</strong>
            <span>账号标识信息</span>
          </div>
        </template>

        <div class="profile-view__info-list">
          <div
            v-for="item in basicInfoItems"
            :key="item.label"
            class="profile-view__info-item"
          >
            <span>{{ item.label }}</span>
            <strong>{{ item.value }}</strong>
          </div>
        </div>
      </el-card>

      <el-card
        class="profile-view__card"
        shadow="never"
      >
        <template #header>
          <div class="profile-view__card-header">
            <strong>访问权限</strong>
            <span>角色摘要与权限分组</span>
          </div>
        </template>

        <div class="profile-view__permission-block">
          <div class="profile-view__tag-group">
            <span class="profile-view__tag-label">角色</span>
            <div class="profile-view__tags">
              <el-tag
                v-for="role in roleDisplayItems"
                :key="role.code"
                round
                effect="plain"
              >
                {{ role.label }}
              </el-tag>
              <span
                v-if="!roleDisplayItems.length"
                class="profile-view__empty"
              >
                暂无角色信息
              </span>
            </div>
          </div>

          <div class="profile-view__permission-summary">
            <article class="profile-view__permission-summary-item">
              <span>权限总数</span>
              <strong>{{ permissionCount }}</strong>
            </article>
            <article class="profile-view__permission-summary-item">
              <span>权限分组</span>
              <strong>{{ permissionGroupCount }}</strong>
            </article>
          </div>

          <div class="profile-view__permission-groups">
            <article
              v-for="group in visiblePermissionGroups"
              :key="group.key"
              class="profile-view__permission-group"
            >
              <div class="profile-view__permission-group-head">
                <strong>{{ group.label }}</strong>
                <span>{{ group.codes.length }} 项</span>
              </div>

              <div class="profile-view__tags">
                <el-tag
                  v-for="action in group.actions"
                  :key="`${group.key}-${action}`"
                  round
                  effect="plain"
                  type="success"
                >
                  {{ formatPermissionActionLabel(action) }}
                </el-tag>
              </div>
            </article>

            <span
              v-if="!permissionGroups.length"
              class="profile-view__empty"
            >
              暂无权限信息
            </span>
          </div>

          <div
            v-if="permissionGroups.length > 6"
            class="profile-view__permission-actions"
          >
            <button
              type="button"
              class="profile-view__toggle"
              @click="permissionsExpanded = !permissionsExpanded"
            >
              {{ permissionsExpanded ? '收起权限分组' : `展开剩余 ${hiddenPermissionGroupCount} 组` }}
            </button>
          </div>
        </div>
      </el-card>

      <el-card
        class="profile-view__card profile-view__card--wide"
        shadow="never"
      >
        <template #header>
          <div class="profile-view__card-header">
            <strong>身份概览</strong>
            <span>基于当前 `me` 数据整理</span>
          </div>
        </template>

        <div class="profile-view__status-grid">
          <article
            v-for="item in identityInfoItems"
            :key="item.label"
            class="profile-view__status-item"
          >
            <span>{{ item.label }}</span>
            <strong>{{ item.value }}</strong>
          </article>
        </div>
      </el-card>
    </div>
  </section>
</template>

<style scoped>
.profile-view {
  display: grid;
  gap: 14px;
}

.profile-view__hero,
.profile-view__card {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  box-shadow: var(--app-shadow);
}

.profile-view__hero-main {
  display: flex;
  align-items: center;
  gap: 18px;
}

.profile-view__avatar {
  display: inline-flex;
  width: 72px;
  height: 72px;
  align-items: center;
  justify-content: center;
  border-radius: 24px;
  background: linear-gradient(135deg, #7aa7ff 0%, #5c7cff 100%);
  color: #fff;
  font-size: 22px;
  font-weight: 800;
  letter-spacing: 1px;
}

.profile-view__hero-copy {
  display: grid;
  gap: 8px;
}

.profile-view__hero-subline {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 16px;
  color: var(--app-text-soft);
  font-size: 12px;
}

.profile-view__hero-heading {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.profile-view__hero-heading h1 {
  margin: 0;
  color: #111827;
  font-size: clamp(24px, 2.8vw, 30px);
  line-height: 1.2;
}

.profile-view__hero-copy p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.profile-view__grid {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.profile-view__card--wide {
  grid-column: 1 / -1;
}

.profile-view__card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.profile-view__card-header strong {
  color: #111827;
  font-size: 16px;
}

.profile-view__card-header span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.profile-view__info-list,
.profile-view__permission-block {
  display: grid;
  gap: 12px;
}

.profile-view__permission-summary {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.profile-view__permission-summary-item {
  display: grid;
  gap: 6px;
  padding: 16px 18px;
  border: 1px solid var(--app-border);
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.profile-view__permission-summary-item span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.profile-view__permission-summary-item strong {
  color: #111827;
  font-size: 18px;
}

.profile-view__info-item,
.profile-view__status-item {
  display: grid;
  gap: 6px;
  padding: 16px 18px;
  border: 1px solid var(--app-border);
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.profile-view__info-item span,
.profile-view__status-item span,
.profile-view__tag-label {
  color: var(--app-text-faint);
  font-size: 12px;
}

.profile-view__info-item strong,
.profile-view__status-item strong {
  color: #111827;
  font-size: 16px;
  line-height: 1.3;
}

.profile-view__tag-group {
  display: grid;
  gap: 10px;
  padding: 16px 18px;
  border: 1px solid var(--app-border);
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.profile-view__tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.profile-view__permission-groups {
  display: grid;
  gap: 12px;
}

.profile-view__permission-group {
  display: grid;
  gap: 10px;
  padding: 16px 18px;
  border: 1px solid var(--app-border);
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.profile-view__permission-group-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.profile-view__permission-group-head strong {
  color: #111827;
  font-size: 14px;
}

.profile-view__permission-group-head span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.profile-view__permission-actions {
  display: flex;
  justify-content: flex-start;
}

.profile-view__toggle {
  height: 34px;
  padding: 0 14px;
  border: 1px solid var(--app-border);
  border-radius: 999px;
  background: #fff;
  color: #334155;
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
}

.profile-view__empty {
  color: var(--app-text-soft);
  font-size: 12px;
}

.profile-view__status-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

:deep(.profile-view__card .el-card__header) {
  border-bottom-color: var(--app-border);
}

@media (max-width: 960px) {
  .profile-view__grid,
  .profile-view__status-grid {
    grid-template-columns: 1fr;
  }

  .profile-view__permission-summary {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .profile-view__hero-main {
    align-items: flex-start;
    flex-direction: column;
  }

  .profile-view__card-header {
    align-items: flex-start;
    flex-direction: column;
  }
}
</style>
