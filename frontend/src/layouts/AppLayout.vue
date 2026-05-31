<script setup lang="ts">
import {
  ArrowDown,
  Bell,
  CollectionTag,
  Compass,
  CreditCard,
  Document,
  Fold,
  Histogram,
  Monitor,
  Setting,
  SwitchButton,
  User,
  Wallet,
} from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

type NavItem = {
  key: string
  label: string
  icon?: unknown
  path?: string
  description?: string
}

type NavSection = {
  title: string
  items: NavItem[]
}

const collapsed = ref(false)
const isHeaderScrolled = ref(false)
const layoutStyle = computed(() => ({
  '--app-header-height': '64px',
  '--app-sidebar-width': collapsed.value ? '60px' : '196px',
}))

const topNavItems: NavItem[] = [
  { key: 'workspace', label: '工作台', path: '/dashboard' },
  { key: 'permission', label: '权限中心' },
  { key: 'organization', label: '组织架构' },
  { key: 'audit', label: '审计中心' },
  { key: 'settings', label: '系统配置', path: '/system' },
  { key: 'profile', label: '个人中心', path: '/profile' },
]

const sidebarSections: NavSection[] = [
  {
    title: '工作台',
    items: [
      {
        key: 'dashboard',
        label: '概览看板',
        icon: Histogram,
        path: '/dashboard',
      },
    ],
  },
  {
    title: '权限管理',
    items: [
      { key: 'users', label: '用户管理', icon: User },
      { key: 'roles', label: '角色管理', icon: CollectionTag },
      { key: 'permissions', label: '权限点管理', icon: Compass },
      { key: 'menus', label: '菜单管理', icon: Document },
    ],
  },
  {
    title: '组织与审计',
    items: [
      { key: 'departments', label: '部门管理', icon: CreditCard },
      { key: 'audit-logs', label: '审计日志', icon: Wallet },
      { key: 'system', label: '系统设置', icon: Setting, path: '/system' },
    ],
  },
  {
    title: '个人中心',
    items: [
      { key: 'profile', label: '个人信息', icon: User, path: '/profile' },
    ],
  },
]

const activePath = computed(() => route.path)
const userDisplayName = computed(() => authStore.displayName)
const userAvatarText = computed(() => authStore.avatarText)
const activeTopNavKey = computed(() => {
  if (route.path.startsWith('/system')) {
    return 'settings'
  }

  if (route.path.startsWith('/profile')) {
    return 'profile'
  }

  if (route.path.startsWith('/dashboard')) {
    return 'workspace'
  }

  return 'workspace'
})

function handleRoute(path?: string, description?: string) {
  if (path) {
    void router.push(path)
    return
  }

  ElMessage.info(description ?? '功能建设中。')
}

async function handleUserCommand(command: string) {
  if (command === 'profile') {
    void router.push('/profile')
    return
  }

  if (command === 'logout') {
    await authStore.logout()
    ElMessage.success('已退出登录。')
    await router.replace('/login')
    return
  }

  const messageMap: Record<string, string> = {
    language: '语言切换稍后接入。',
  }

  ElMessage.info(messageMap[command] ?? '功能规划中。')
}

function handleTenantCommand(command: string) {
  const tenantMap: Record<string, string> = {
    'tenant-hq': '已切换到总部租户。',
    'tenant-demo': '已切换到演示租户。',
  }

  ElMessage.success(tenantMap[command] ?? '租户切换功能建设中。')
}

function handleContentScroll(event: Event) {
  const target = event.target as HTMLElement | null
  isHeaderScrolled.value = (target?.scrollTop ?? 0) > 8
}
</script>

<template>
  <div
    class="app-layout"
    :style="layoutStyle"
  >
    <header
      class="app-layout__header"
      :class="{ 'app-layout__header--scrolled': isHeaderScrolled }"
    >
      <div class="app-layout__header-left">
        <button
          type="button"
          class="app-layout__brand"
          @click="handleRoute('/dashboard')"
        >
          <span class="app-layout__brand-mark">R</span>
          <span class="app-layout__brand-name">r-admin</span>
        </button>

        <nav class="app-layout__top-nav">
          <button
            v-for="item in topNavItems"
            :key="item.key"
            type="button"
            class="app-layout__top-nav-item"
            :class="{ 'is-active': item.key === activeTopNavKey }"
            @click="handleRoute(item.path, item.description)"
          >
            {{ item.label }}
          </button>
        </nav>
      </div>

      <div class="app-layout__toolbar">
        <button
          type="button"
          class="app-layout__toolbar-button"
          @click="handleRoute(undefined, '通知中心仍在建设中。')"
        >
          <el-badge :value="2">
            <el-icon><Bell /></el-icon>
          </el-badge>
        </button>

        <button
          type="button"
          class="app-layout__toolbar-button"
          @click="handleRoute(undefined, '桌面布局切换会在后续版本接入。')"
        >
          <el-icon><Monitor /></el-icon>
        </button>

        <el-dropdown
          trigger="click"
          @command="handleTenantCommand"
        >
          <button
            type="button"
            class="app-layout__tenant"
          >
            <span class="app-layout__tenant-label">租户</span>
            <span class="app-layout__tenant-value">默认租户</span>
            <el-icon><ArrowDown /></el-icon>
          </button>

          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="tenant-hq">总部租户</el-dropdown-item>
              <el-dropdown-item command="tenant-demo">演示租户</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

        <el-dropdown
          trigger="click"
          @command="handleUserCommand"
        >
          <button
            type="button"
            class="app-layout__toolbar-button"
          >
            <span class="app-layout__toolbar-glyph">{{ userAvatarText.slice(0, 1) }}</span>
          </button>

          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="language">简体中文</el-dropdown-item>
              <el-dropdown-item command="language">English</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

        <el-dropdown
          trigger="click"
          @command="handleUserCommand"
        >
          <button
            type="button"
            class="app-layout__user"
          >
            <span class="app-layout__user-avatar">{{ userAvatarText }}</span>
            <span class="app-layout__user-name">{{ userDisplayName }}</span>
            <el-icon><ArrowDown /></el-icon>
          </button>

          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="profile">
                <el-icon><Setting /></el-icon>
                <span>个人设置</span>
              </el-dropdown-item>
              <el-dropdown-item command="logout">
                <el-icon><SwitchButton /></el-icon>
                <span>退出</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </header>

    <aside
      class="app-layout__aside"
      :class="{ 'app-layout__aside--collapsed': collapsed }"
    >
      <div class="app-layout__sections">
        <section
          v-for="section in sidebarSections"
          :key="section.title"
          class="app-layout__section"
        >
          <p
            v-show="!collapsed"
            class="app-layout__section-title"
          >
            {{ section.title }}
          </p>

          <button
            v-for="item in section.items"
            :key="item.key"
            type="button"
            class="app-layout__nav-item"
            :class="{ 'is-active': item.path === activePath }"
            :title="collapsed ? item.label : ''"
            @click="handleRoute(item.path, item.description)"
          >
            <el-icon class="app-layout__nav-icon"><component :is="item.icon" /></el-icon>
            <span v-show="!collapsed">{{ item.label }}</span>
          </button>

        </section>
      </div>

      <button
        type="button"
        class="app-layout__collapse"
        @click="collapsed = !collapsed"
      >
        <el-icon class="app-layout__collapse-icon"><Fold /></el-icon>
        <span v-show="!collapsed">收起侧边栏</span>
      </button>
    </aside>

    <div
      class="app-layout__content"
      @scroll="handleContentScroll"
    >
      <main class="app-layout__main">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  height: 100vh;
  background: var(--app-bg);
  overflow: hidden;
}

.app-layout__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 14px 18px;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: var(--app-header-height);
  background: rgba(255, 255, 255, 0.96);
  z-index: 110;
  transition:
    background 0.2s ease,
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    backdrop-filter 0.2s ease;
}

.app-layout__header--scrolled {
  background: rgba(255, 255, 255, 0.82);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  box-shadow: 0 8px 30px rgba(15, 23, 42, 0.04);
}

.app-layout__header-left {
  display: flex;
  align-items: center;
  gap: 22px;
  min-width: 0;
}

.app-layout__brand {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--app-text);
  cursor: pointer;
}

.app-layout__brand-mark {
  display: inline-flex;
  width: 34px;
  height: 34px;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  border: 1px solid #f0c9cf;
  background:
    radial-gradient(circle at 30% 30%, rgba(255, 255, 255, 0.95), rgba(255, 236, 240, 0.92)),
    linear-gradient(135deg, #fde6eb, #fff8f9);
  color: #d26a7d;
  font-size: 16px;
  font-weight: 700;
}

.app-layout__brand-name {
  font-size: 16px;
  font-weight: 700;
}

.app-layout__top-nav {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.app-layout__top-nav-item {
  padding: 8px 9px;
  border: 0;
  background: transparent;
  color: var(--app-text);
  font-size: 15px;
  font-weight: 700;
  cursor: pointer;
}

.app-layout__top-nav-item.is-active,
.app-layout__top-nav-item:hover {
  color: var(--app-primary);
}

.app-layout__toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.app-layout__toolbar-button,
.app-layout__tenant,
.app-layout__user {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  height: 36px;
  padding: 0 12px;
  border: 1px solid var(--app-border);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.72);
  color: var(--app-text-soft);
  cursor: pointer;
  transition:
    border-color 0.2s ease,
    color 0.2s ease,
    background 0.2s ease;
}

.app-layout__toolbar-button {
  width: 36px;
  padding: 0;
}

.app-layout__tenant {
  gap: 8px;
  padding: 0 12px;
}

.app-layout__toolbar-button:hover,
.app-layout__tenant:hover,
.app-layout__user:hover {
  border-color: #d7dfeb;
  color: var(--app-text);
  background: #f7f9fc;
}

.app-layout__toolbar-glyph {
  font-size: 15px;
  font-weight: 700;
}

.app-layout__tenant-label {
  color: var(--app-text-soft);
  font-size: 12px;
}

.app-layout__tenant-value {
  color: var(--app-text);
  font-size: 13px;
  font-weight: 500;
}

.app-layout__user-avatar {
  display: inline-flex;
  width: 22px;
  height: 22px;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: #7183d8;
  color: #fff;
  font-size: 12px;
  font-weight: 700;
}

.app-layout__user-name {
  color: var(--app-text);
  font-size: 13px;
  font-weight: 500;
}

.app-layout__aside {
  box-sizing: border-box;
  display: flex;
  width: var(--app-sidebar-width);
  min-width: var(--app-sidebar-width);
  flex-direction: column;
  padding: 12px 8px 12px;
  position: fixed;
  top: var(--app-header-height);
  left: 0;
  bottom: 0;
  background: var(--app-surface-strong);
  overflow-y: auto;
  overflow-x: hidden;
  scrollbar-gutter: stable;
  z-index: 100;
  transition: width 0.25s ease;
}

.app-layout__sections {
  flex: 1;
  display: grid;
  gap: 8px;
}

.app-layout__section {
  display: grid;
  gap: 1px;
}

.app-layout__section-title {
  margin: 0;
  padding: 4px 15px 8px;
  color: var(--app-text-faint);
  font-size: 12px;
  line-height: 1;
  font-weight: 400;
}

.app-layout__nav-item {
  box-sizing: border-box;
  display: flex;
  align-items: center;
  gap: 10px;
  width: calc(100% - 16px);
  max-width: 164px;
  height: 30px;
  margin: 3px 8px;
  padding: 0 15px;
  border: 0;
  border-radius: 10px;
  background: transparent;
  color: var(--app-text-soft);
  font-size: 14px;
  font-weight: 400;
  line-height: 1;
  cursor: pointer;
  text-align: left;
  transition:
    background 0.2s ease,
    color 0.2s ease,
    transform 0.2s ease;
}

.app-layout__nav-item:hover {
  color: var(--app-text);
  background: rgba(31, 122, 255, 0.04);
}

.app-layout__nav-item.is-active {
  background: #e8f2ff;
  color: var(--app-primary);
  font-weight: 400;
}

.app-layout__nav-icon {
  display: inline-flex;
  width: 16px;
  min-width: 16px;
  justify-content: center;
  font-size: 16px;
}

.app-layout__aside--collapsed .app-layout__sections {
  gap: 8px;
}

.app-layout__aside--collapsed .app-layout__nav-item {
  justify-content: center;
  width: 44px;
  max-width: 44px;
  height: 44px;
  margin: 0 8px 4px;
  padding: 0;
}

.app-layout__aside--collapsed .app-layout__collapse {
  width: 44px;
  max-width: 44px;
  height: 44px;
  margin: 0 8px 4px;
  padding: 0;
  border-radius: 12px;
}

.app-layout__collapse {
  box-sizing: border-box;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: calc(100% - 16px);
  max-width: 164px;
  height: 30px;
  margin: 8px 8px 0;
  border: 1px solid var(--app-border);
  border-radius: 999px;
  background: #fff;
  color: var(--app-text-soft);
  font-size: 12px;
  font-weight: 600;
  line-height: 1;
  cursor: pointer;
}

.app-layout__collapse-icon {
  transition: transform 0.25s ease;
}

.app-layout__aside--collapsed .app-layout__collapse-icon {
  transform: rotate(180deg);
}

.app-layout__content {
  height: calc(100vh - var(--app-header-height));
  margin-top: var(--app-header-height);
  margin-left: var(--app-sidebar-width);
  overflow-y: auto;
  overscroll-behavior: contain;
  scrollbar-gutter: stable;
  -ms-overflow-style: none;
  scrollbar-width: none;
  transition: margin-left 0.25s ease;
}

.app-layout__content::-webkit-scrollbar {
  width: 0;
  height: 0;
  display: none;
}

.app-layout__main {
  min-width: 0;
  min-height: 100%;
  padding: 14px 18px 20px;
}

@media (max-width: 1180px) {
  .app-layout {
    height: auto;
    overflow: visible;
  }

  .app-layout__header,
  .app-layout__header-left {
    flex-direction: column;
    align-items: stretch;
  }

  .app-layout__top-nav,
  .app-layout__toolbar {
    justify-content: flex-start;
  }

  .app-layout__header {
    position: static;
    height: auto;
  }

  .app-layout__aside {
    width: auto;
    position: static;
    border-right: 0;
    border-bottom: 1px solid var(--app-border);
    overflow-y: visible;
  }

  .app-layout__content {
    height: auto;
    margin-top: 0;
    margin-left: 0;
    overflow-y: visible;
  }
}

@media (max-width: 720px) {
  .app-layout__header {
    padding: 16px;
  }

  .app-layout__main {
    padding: 16px;
  }

  .app-layout__top-nav {
    gap: 2px;
  }

  .app-layout__top-nav-item {
    padding: 6px 8px;
    font-size: 14px;
  }

  .app-layout__user-name {
    display: none;
  }

  .app-layout__tenant-label {
    display: none;
  }
}
</style>
