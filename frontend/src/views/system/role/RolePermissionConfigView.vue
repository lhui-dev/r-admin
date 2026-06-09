<script setup lang="ts">
import {
  ArrowLeft,
  CircleCheck,
  RefreshRight,
} from '@element-plus/icons-vue'
import { ElMessage, type ElTree, type TreeNodeData } from 'element-plus'
import { computed, nextTick, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import PageContent from '@/base-ui/page-content/PageContent.vue'
import {
  fetchSystemRolePermissionConfig,
  updateSystemRolePermissions,
  type RoleDetailData,
  type RolePermissionTreeNode,
} from '@/api/system-role'

const route = useRoute()
const router = useRouter()

const treeRef = ref<InstanceType<typeof ElTree>>()
const loading = ref(false)
const submitLoading = ref(false)
const roleDetail = ref<RoleDetailData | null>(null)
const permissionTree = ref<RolePermissionTreeNode[]>([])
const checkedPermissionIds = ref<string[]>([])
const initialCheckedPermissionIds = ref<string[]>([])
const treeKeyword = ref('')

const roleId = computed(() => Number(route.params.roleId))
const roleName = computed(() => roleDetail.value?.name ?? '角色权限配置')
const checkedLeafIds = computed(() => checkedPermissionIds.value)
const checkedLeafIdSet = computed(() => new Set(checkedLeafIds.value))
const allLeafPermissionIds = computed(() => flattenPermissionNodes(permissionTree.value).map(node => node.id))
const selectedPermissionCount = computed(() => checkedLeafIds.value.length)
const selectedModuleCount = computed(() => {
  const moduleIds = new Set<string>()

  checkedLeafIds.value.forEach((permissionId) => {
    const path = findNodePath(permissionTree.value, permissionId)
    if (path.length > 0) {
      moduleIds.add(path[0].id)
    }
  })

  return moduleIds.size
})
const selectedPermissionLabels = computed(() => {
  const labels = new Map<string, string>()

  flattenPermissionNodes(permissionTree.value).forEach((node) => {
    labels.set(node.id, node.name)
  })

  return checkedLeafIds.value
    .map(permissionId => ({
      id: permissionId,
      name: labels.get(permissionId) ?? permissionId,
    }))
})
const hasPendingChanges = computed(() => {
  const current = [...checkedLeafIds.value].sort().join('|')
  const initial = [...initialCheckedPermissionIds.value].sort().join('|')
  return current !== initial
})
const roleDataScopeLabel = computed(() => {
  const dataScope = roleDetail.value?.data_scope
  return dataScope ? resolveDataScopeLabel(dataScope) : '未加载'
})
const roleStatusLabel = computed(() => (roleDetail.value?.status === 1 ? '启用中' : '已禁用'))
const roleStatusTagType = computed(() => (roleDetail.value?.status === 1 ? 'success' : 'danger'))
const roleTypeLabel = computed(() => (roleDetail.value?.is_builtin ? '内置角色' : '自定义角色'))
const isAllLeafPermissionsSelected = computed(() => {
  return allLeafPermissionIds.value.length > 0
    && selectedPermissionCount.value === allLeafPermissionIds.value.length
})
const selectionCoverageText = computed(() => {
  const total = allLeafPermissionIds.value.length
  if (!total) {
    return '等待权限树加载'
  }

  return `已覆盖 ${selectedPermissionCount.value} / ${total} 个可选权限点`
})
const selectionChangeText = computed(() => {
  if (!hasPendingChanges.value) {
    return '当前选择已与最近一次保存状态保持一致。'
  }

  return `当前有 ${selectedPermissionCount.value} 个权限点待保存，请确认后提交。`
})
const saveStateText = computed(() => {
  if (submitLoading.value) {
    return '正在提交新的权限配置...'
  }

  if (hasPendingChanges.value) {
    return '存在未保存变更'
  }

  return '已保存到当前原型数据'
})
const saveStateTagType = computed(() => {
  if (submitLoading.value) {
    return 'warning'
  }

  return hasPendingChanges.value ? 'danger' : 'success'
})
const lastSavedAt = computed(() => roleDetail.value?.updated_at ?? '未加载')

async function loadPermissionConfig() {
  if (!Number.isFinite(roleId.value) || roleId.value <= 0) {
    ElMessage.error('角色参数无效，无法加载权限配置')
    void router.replace('/system/role')
    return
  }

  loading.value = true
  roleDetail.value = null
  permissionTree.value = []
  checkedPermissionIds.value = []
  initialCheckedPermissionIds.value = []

  try {
    const response = await fetchSystemRolePermissionConfig(roleId.value)
    roleDetail.value = response.data.role
    permissionTree.value = response.data.permission_tree
    checkedPermissionIds.value = [...response.data.checked_permission_ids]
    initialCheckedPermissionIds.value = [...response.data.checked_permission_ids]

    await nextTick()
    treeRef.value?.setCheckedKeys(response.data.checked_permission_ids)
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '权限配置加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleTreeCheck() {
  const keyword = treeKeyword.value.trim()
  if (!keyword) {
    checkedPermissionIds.value = normalizeCheckedLeafIds()
    return
  }

  const checkedKeys = (treeRef.value?.getCheckedKeys(false) ?? []).map(key => String(key))
  const checkedKeySet = new Set(checkedKeys)
  const visibleLeafIds = collectLeafPermissionIds(permissionTree.value, keyword)
  const nextCheckedIds = new Set(checkedLeafIds.value)

  visibleLeafIds.forEach((permissionId) => {
    if (checkedKeySet.has(permissionId)) {
      nextCheckedIds.add(permissionId)
      return
    }

    nextCheckedIds.delete(permissionId)
  })

  checkedPermissionIds.value = sortPermissionIdsByTreeOrder([...nextCheckedIds])
}

function handleResetChecked() {
  if (!hasPendingChanges.value || submitLoading.value) {
    return
  }

  checkedPermissionIds.value = [...initialCheckedPermissionIds.value]
  treeRef.value?.setCheckedKeys(initialCheckedPermissionIds.value)
  ElMessage.success('已恢复到当前角色已保存的权限配置')
}

async function handleSave() {
  if (!hasPendingChanges.value) {
    return
  }

  submitLoading.value = true

  try {
    const nextCheckedIds = [...checkedLeafIds.value]
    const response = await updateSystemRolePermissions(roleId.value, {
      permission_ids: nextCheckedIds,
    })
    const savedPermissionIds = response.data.permissions.map(item => item.id)

    roleDetail.value = response.data
    checkedPermissionIds.value = savedPermissionIds
    initialCheckedPermissionIds.value = [...savedPermissionIds]
    await nextTick()
    treeRef.value?.setCheckedKeys(savedPermissionIds)
    ElMessage.success('权限配置保存成功')
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '权限配置保存失败'))
  }
  finally {
    submitLoading.value = false
  }
}

function handleBack() {
  void router.push('/system/role')
}

function handleCheckAll() {
  if (!allLeafPermissionIds.value.length) {
    return
  }

  checkedPermissionIds.value = [...allLeafPermissionIds.value]
  treeRef.value?.setCheckedKeys(allLeafPermissionIds.value)
}

function handleClearAll() {
  checkedPermissionIds.value = []
  treeRef.value?.setCheckedKeys([])
}

function handleTreeFilter(value: string) {
  treeRef.value?.filter(value.trim())
}

function filterTreeNode(value: string, data: TreeNodeData) {
  if (!value) {
    return true
  }

  return String((data as RolePermissionTreeNode).name).includes(value)
}

function isCheckedLeafNode(node: RolePermissionTreeNode) {
  return !node.children?.length && checkedLeafIdSet.value.has(node.id)
}

function resolveNodeTypeLabel(type: RolePermissionTreeNode['type']) {
  const map: Record<RolePermissionTreeNode['type'], string> = {
    module: '模块',
    menu: '菜单',
    button: '按钮',
    api: '接口',
  }

  return map[type]
}

function resolveDataScopeLabel(value: string) {
  const map: Record<string, string> = {
    all: '全部数据',
    tenant: '租户内数据',
    department: '本部门数据',
    custom: '自定义数据',
    self: '仅本人数据',
  }

  return map[value] ?? value
}

function normalizeCheckedLeafIds() {
  const checkedKeys = (treeRef.value?.getCheckedKeys(false) ?? []) as string[]
  const checkedLeafSet = new Set<string>()
  const leafIds = new Set(flattenPermissionNodes(permissionTree.value).map(node => node.id))

  checkedKeys.forEach((key) => {
    if (leafIds.has(key)) {
      checkedLeafSet.add(key)
    }
  })

  return [...checkedLeafSet]
}

function flattenPermissionNodes(nodes: RolePermissionTreeNode[]): Array<{ id: string, name: string }> {
  return nodes.flatMap((node) => {
    if (!node.children?.length) {
      return [{ id: node.id, name: node.name }]
    }

    return flattenPermissionNodes(node.children)
  })
}

function collectLeafPermissionIds(nodes: RolePermissionTreeNode[], keyword = ''): string[] {
  return nodes.flatMap((node) => {
    if (!node.children?.length) {
      if (!keyword || node.name.includes(keyword) || node.id.includes(keyword)) {
        return [node.id]
      }

      return []
    }

    return collectLeafPermissionIds(node.children, keyword)
  })
}

function sortPermissionIdsByTreeOrder(permissionIds: string[]) {
  const permissionIdSet = new Set(permissionIds)
  const orderedPermissionIds = allLeafPermissionIds.value.filter(permissionId => permissionIdSet.has(permissionId))
  const orderedPermissionIdSet = new Set(orderedPermissionIds)
  const outOfTreePermissionIds = permissionIds.filter(permissionId => !orderedPermissionIdSet.has(permissionId))

  return [...orderedPermissionIds, ...outOfTreePermissionIds]
}

function findNodePath(nodes: RolePermissionTreeNode[], targetId: string, path: RolePermissionTreeNode[] = []): RolePermissionTreeNode[] {
  for (const node of nodes) {
    const nextPath = [...path, node]
    if (node.id === targetId) {
      return nextPath
    }

    if (node.children?.length) {
      const result = findNodePath(node.children, targetId, nextPath)
      if (result.length) {
        return result
      }
    }
  }

  return []
}

function resolveErrorMessage(error: unknown, fallback: string) {
  if (typeof error === 'object' && error !== null) {
    const message = String((error as { message?: unknown }).message ?? '')
    if (message) {
      return message
    }
  }

  return fallback
}

watch(roleId, () => {
  void loadPermissionConfig()
}, { immediate: true })
</script>

<template>
  <PageContent
    eyebrow="Role Permission"
    title="权限配置"
    :description="`围绕角色 “${roleName}” 配置菜单与权限点，使用真实后端数据完成授权闭环。`"
  >
    <template #actions>
      <el-button @click="handleBack">
        <el-icon><ArrowLeft /></el-icon>
        <span>返回角色管理</span>
      </el-button>
      <el-button
        :disabled="loading || submitLoading || !hasPendingChanges"
        @click="handleResetChecked"
      >
        <el-icon><RefreshRight /></el-icon>
        <span>重置选择</span>
      </el-button>
      <el-button
        type="primary"
        :disabled="loading || submitLoading || !hasPendingChanges"
        :loading="submitLoading"
        @click="handleSave"
      >
        <el-icon><CircleCheck /></el-icon>
        <span>保存配置</span>
      </el-button>
    </template>

    <section class="role-permission__overview">
      <article class="role-permission__overview-card role-permission__overview-card--primary">
        <span>当前角色</span>
        <strong>{{ roleDetail?.name ?? '加载中' }}</strong>
        <p>{{ roleDetail?.code ?? '等待角色数据加载' }}</p>
        <div class="role-permission__overview-meta">
          <el-tag
            round
            size="small"
            :type="roleStatusTagType"
          >
            {{ roleStatusLabel }}
          </el-tag>
          <el-tag
            round
            size="small"
            type="info"
          >
            {{ roleTypeLabel }}
          </el-tag>
        </div>
      </article>

      <article class="role-permission__overview-card role-permission__overview-card--success">
        <span>已选权限点</span>
        <strong>{{ selectedPermissionCount }}</strong>
        <p>{{ selectionCoverageText }}</p>
      </article>

      <article class="role-permission__overview-card role-permission__overview-card--warning">
        <span>覆盖模块数</span>
        <strong>{{ selectedModuleCount }}</strong>
        <p>用于快速判断当前角色能力覆盖范围。</p>
      </article>
    </section>

    <section
      v-loading="loading"
      class="role-permission__workspace"
    >
      <article class="role-permission__tree-panel role-permission__panel">
        <header class="role-permission__panel-header">
          <div>
            <strong>权限树</strong>
            <p>先选择菜单与按钮权限，再保存到当前角色。</p>
          </div>
          <div class="role-permission__panel-tags">
            <el-tag
              round
              type="primary"
            >
              原型阶段
            </el-tag>
            <el-tag
              round
              type="info"
            >
              {{ roleDataScopeLabel }}
            </el-tag>
          </div>
        </header>

        <div class="role-permission__tree-toolbar">
          <el-input
            v-model="treeKeyword"
            class="role-permission__tree-search"
            placeholder="筛选模块 / 菜单 / 权限点"
            clearable
            @input="handleTreeFilter"
          />

          <div class="role-permission__tree-actions">
            <el-button
              text
              :disabled="loading || isAllLeafPermissionsSelected || !allLeafPermissionIds.length"
              @click="handleCheckAll"
            >
              全选叶子权限
            </el-button>
            <el-button
              text
              :disabled="loading || !checkedLeafIds.length"
              @click="handleClearAll"
            >
              清空选择
            </el-button>
          </div>
        </div>

        <div class="role-permission__tree-shell">
          <el-tree
            ref="treeRef"
            node-key="id"
            show-checkbox
            default-expand-all
            :data="permissionTree"
            :props="{ label: 'name', children: 'children' }"
            :filter-node-method="filterTreeNode"
            @check="handleTreeCheck"
          >
            <template #default="{ data }">
              <div
                class="role-permission__tree-node"
                :class="{ 'role-permission__tree-node--checked': isCheckedLeafNode(data) }"
              >
                <span>{{ data.name }}</span>
                <em>{{ resolveNodeTypeLabel(data.type) }}</em>
                <i
                  v-if="isCheckedLeafNode(data)"
                  class="role-permission__tree-node-state"
                >
                  已选
                </i>
              </div>
            </template>
          </el-tree>
        </div>
      </article>

      <aside class="role-permission__summary">
        <article class="role-permission__summary-card role-permission__panel">
          <header class="role-permission__panel-header">
            <div>
              <strong>角色摘要</strong>
              <p>帮助你在配置时确认角色边界与状态。</p>
            </div>
          </header>

          <dl class="role-permission__summary-list">
            <div>
              <dt>角色名称</dt>
              <dd>{{ roleDetail?.name ?? '未加载' }}</dd>
            </div>
            <div>
              <dt>角色编码</dt>
              <dd>{{ roleDetail?.code ?? '未加载' }}</dd>
            </div>
            <div>
              <dt>数据范围</dt>
              <dd>{{ roleDataScopeLabel }}</dd>
            </div>
            <div>
              <dt>角色类型</dt>
              <dd>{{ roleTypeLabel }}</dd>
            </div>
            <div>
              <dt>角色状态</dt>
              <dd>{{ roleStatusLabel }}</dd>
            </div>
            <div>
              <dt>最近更新</dt>
              <dd>{{ roleDetail?.updated_at ?? '未加载' }}</dd>
            </div>
          </dl>
        </article>

        <article class="role-permission__summary-card role-permission__panel">
          <header class="role-permission__panel-header">
            <div>
              <strong>当前选择</strong>
              <p>勾选结果会作为保存时的最终权限集合。</p>
            </div>
            <div class="role-permission__summary-status">
              <span
                class="role-permission__change-badge"
                :class="{ 'role-permission__change-badge--active': hasPendingChanges }"
              >
                {{ hasPendingChanges ? '有未保存变更' : '已与保存状态一致' }}
              </span>
              <el-tag
                round
                size="small"
                :type="saveStateTagType"
              >
                {{ saveStateText }}
              </el-tag>
            </div>
          </header>

          <div class="role-permission__selected">
            <div class="role-permission__selected-stats">
              <article>
                <span>已选权限点</span>
                <strong>{{ selectedPermissionCount }}</strong>
              </article>
              <article>
                <span>覆盖模块</span>
                <strong>{{ selectedModuleCount }}</strong>
              </article>
            </div>

            <p class="role-permission__selected-progress">
              {{ selectionCoverageText }}
            </p>

            <div class="role-permission__save-hint">
              <strong>保存状态</strong>
              <p>{{ selectionChangeText }}</p>
              <span>最近保存时间：{{ lastSavedAt }}</span>
            </div>

            <div class="role-permission__selected-tags">
              <template v-if="selectedPermissionLabels.length">
                <el-tag
                  v-for="item in selectedPermissionLabels"
                  :key="item.id"
                  round
                  type="info"
                >
                  {{ item.name }}
                </el-tag>
              </template>
              <p v-else>
                当前还没有勾选任何叶子权限点。
              </p>
            </div>
          </div>
        </article>

        <article class="role-permission__summary-card role-permission__panel">
          <header class="role-permission__panel-header">
            <div>
              <strong>后续接入说明</strong>
              <p>当前页面已经接入真实角色权限接口，后续继续增强授权规则。</p>
            </div>
          </header>

          <ul class="role-permission__notes">
            <li>权限树以菜单层级为主，按钮和接口权限作为可授权叶子节点。</li>
            <li>保存后以后端返回的最新权限集合刷新页面状态，避免旧数据残留。</li>
            <li>后续可以继续补充权限变更审计、差异预览和批量授权能力。</li>
          </ul>
        </article>
      </aside>
    </section>
  </PageContent>
</template>

<style scoped>
.role-permission__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.role-permission__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  position: relative;
  overflow: hidden;
}

.role-permission__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
}

.role-permission__overview-card--primary::before {
  background: var(--app-primary);
}

.role-permission__overview-card--success::before {
  background: var(--app-success);
}

.role-permission__overview-card--warning::before {
  background: var(--app-warning);
}

.role-permission__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.role-permission__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.role-permission__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.role-permission__overview-meta {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.role-permission__workspace {
  display: grid;
  gap: 14px;
  grid-template-columns: minmax(0, 1.45fr) minmax(300px, 0.9fr);
  align-items: start;
}

.role-permission__panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
}

.role-permission__tree-panel,
.role-permission__summary-card {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.role-permission__summary {
  display: grid;
  gap: 14px;
}

.role-permission__panel-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.role-permission__panel-header strong {
  color: #111827;
  font-size: 16px;
}

.role-permission__panel-header p {
  margin: 6px 0 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.role-permission__panel-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.role-permission__tree-toolbar {
  display: grid;
  gap: 12px;
}

.role-permission__tree-search {
  width: 100%;
}

.role-permission__tree-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.role-permission__tree-shell {
  min-height: 520px;
  max-height: 640px;
  padding: 8px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-md);
  background: var(--app-surface-soft);
  overflow: auto;
}

.role-permission__tree-node {
  display: inline-flex;
  min-width: 0;
  align-items: center;
  gap: 10px;
  padding: 3px 0;
}

.role-permission__tree-node span {
  color: #111827;
  font-size: 14px;
}

.role-permission__tree-node--checked span {
  color: var(--app-primary);
  font-weight: 600;
}

.role-permission__tree-node em {
  display: inline-flex;
  align-items: center;
  height: 22px;
  padding: 0 8px;
  border-radius: 999px;
  background: #eef3fb;
  color: #64748b;
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.role-permission__tree-node-state {
  display: inline-flex;
  align-items: center;
  height: 22px;
  padding: 0 8px;
  border-radius: 999px;
  background: color-mix(in srgb, var(--app-primary) 10%, white);
  color: var(--app-primary);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.role-permission__summary-list {
  display: grid;
  gap: 12px;
  margin: 0;
}

.role-permission__summary-list div {
  display: grid;
  gap: 4px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.role-permission__summary-list dt {
  color: var(--app-text-faint);
  font-size: 12px;
}

.role-permission__summary-list dd {
  margin: 0;
  color: #111827;
  font-size: 14px;
  line-height: 1.6;
  font-weight: 600;
}

.role-permission__change-badge {
  display: inline-flex;
  min-height: 30px;
  align-items: center;
  padding: 0 12px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 12px;
  font-weight: 600;
}

.role-permission__change-badge--active {
  background: #fff4db;
  color: #b45309;
}

.role-permission__summary-status {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.role-permission__selected {
  display: grid;
  gap: 14px;
}

.role-permission__save-hint {
  display: grid;
  gap: 4px;
  padding: 14px;
  border-radius: 14px;
  background: color-mix(in srgb, var(--app-primary) 5%, white);
}

.role-permission__save-hint strong {
  color: #111827;
  font-size: 13px;
}

.role-permission__save-hint p,
.role-permission__save-hint span {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-permission__selected-progress {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-permission__selected-stats {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.role-permission__selected-stats article {
  display: grid;
  gap: 6px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.role-permission__selected-stats span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.role-permission__selected-stats strong {
  color: #111827;
  font-size: 22px;
}

.role-permission__selected-tags {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.role-permission__selected-tags p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
}

.role-permission__notes {
  display: grid;
  gap: 10px;
  margin: 0;
  padding-left: 18px;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.8;
}

:deep(.role-permission__tree-shell .el-tree) {
  background: transparent;
}

:deep(.role-permission__tree-shell .el-tree-node__content) {
  height: 36px;
  border-radius: 10px;
}

:deep(.role-permission__tree-shell .el-tree-node__content:hover) {
  background: rgba(31, 122, 255, 0.06);
}

@media (max-width: 1180px) {
  .role-permission__workspace {
    grid-template-columns: 1fr;
  }

  .role-permission__panel-header {
    flex-direction: column;
  }

  .role-permission__panel-tags {
    justify-content: flex-start;
  }
}

@media (max-width: 900px) {
  .role-permission__overview,
  .role-permission__selected-stats {
    grid-template-columns: 1fr;
  }
}
</style>
