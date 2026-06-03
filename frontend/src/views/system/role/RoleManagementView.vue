<script setup lang="ts">
import {
  CircleCheck,
  EditPen,
  Lock,
  Plus,
  RefreshRight,
  Search,
  View,
} from '@element-plus/icons-vue'
import {
  ElMessage,
  ElMessageBox,
  type FormInstance,
  type FormRules,
} from 'element-plus'
import { computed, onMounted, reactive, ref } from 'vue'

import BaseTable from '@/base-ui/base-table/BaseTable.vue'
import PageContent from '@/base-ui/page-content/PageContent.vue'
import PageModal from '@/base-ui/page-modal/PageModal.vue'
import PageSearch from '@/base-ui/page-search/PageSearch.vue'
import {
  createSystemRole,
  fetchSystemRoleDetail,
  fetchSystemRoles,
  updateSystemRole,
  updateSystemRoleStatus,
  type CreateRolePayload,
  type RoleDetailData,
  type RoleListItem,
  type UpdateRolePayload,
} from '@/api/system-role'

type RoleFormModel = {
  code: string
  name: string
  status: number
  dataScope: string
  sort: number
  remark: string
}

const dataScopeOptions = [
  { label: '全部数据', value: 'all' },
  { label: '租户内数据', value: 'tenant' },
  { label: '本部门数据', value: 'department' },
  { label: '自定义数据', value: 'custom' },
  { label: '仅本人数据', value: 'self' },
]

const statusOptions = [
  { label: '启用', value: 1 },
  { label: '禁用', value: 0 },
]

const filters = reactive({
  keyword: '',
  status: undefined as number | undefined,
  dataScope: undefined as string | undefined,
})

const pagination = reactive({
  page: 1,
  pageSize: 10,
  total: 0,
})

const roleList = ref<RoleListItem[]>([])
const loading = ref(false)
const detailVisible = ref(false)
const detailLoading = ref(false)
const currentDetail = ref<RoleDetailData | null>(null)
const modalVisible = ref(false)
const modalLoading = ref(false)
const submitLoading = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingRoleId = ref<number | null>(null)
const formRef = ref<FormInstance>()

const form = reactive<RoleFormModel>(createDefaultForm())

const formRules: FormRules<RoleFormModel> = {
  code: [
    { required: true, message: '请输入角色编码', trigger: 'blur' },
  ],
  name: [
    { required: true, message: '请输入角色名称', trigger: 'blur' },
  ],
  dataScope: [
    { required: true, message: '请选择数据范围', trigger: 'change' },
  ],
}

const isCreateMode = computed(() => modalMode.value === 'create')
const modalTitle = computed(() => (isCreateMode.value ? '新增角色' : '编辑角色'))
const enabledCount = computed(() => roleList.value.filter(item => item.status === 1).length)
const disabledCount = computed(() => roleList.value.filter(item => item.status === 0).length)
const builtinCount = computed(() => roleList.value.filter(item => item.is_builtin).length)
const pageCount = computed(() => Math.max(1, Math.ceil(pagination.total / pagination.pageSize)))
const activeFilterCount = computed(() => [filters.keyword, filters.status, filters.dataScope].filter(value => value !== '' && value !== undefined).length)
const hasActiveFilters = computed(() => activeFilterCount.value > 0)
const currentEditingCode = computed(() => {
  if (!editingRoleId.value) {
    return ''
  }

  return roleList.value.find(item => item.id === editingRoleId.value)?.code ?? ''
})

function createDefaultForm(): RoleFormModel {
  return {
    code: '',
    name: '',
    status: 1,
    dataScope: 'tenant',
    sort: 100,
    remark: '',
  }
}

function resetForm() {
  Object.assign(form, createDefaultForm())
  editingRoleId.value = null
  formRef.value?.clearValidate()
}

async function loadRoles() {
  loading.value = true

  try {
    const response = await fetchSystemRoles({
      page: pagination.page,
      page_size: pagination.pageSize,
      keyword: filters.keyword || undefined,
      status: filters.status,
      data_scope: filters.dataScope,
    })

    roleList.value = response.data.items
    pagination.total = response.data.pagination.total
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '角色列表加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleSearch() {
  pagination.page = 1
  void loadRoles()
}

function handleReset() {
  filters.keyword = ''
  filters.status = undefined
  filters.dataScope = undefined
  pagination.page = 1
  void loadRoles()
}

function handlePageChange(page: number) {
  pagination.page = page
  void loadRoles()
}

function handlePageSizeChange(pageSize: number) {
  pagination.page = 1
  pagination.pageSize = pageSize
  void loadRoles()
}

async function handleView(roleId: number) {
  detailVisible.value = true
  detailLoading.value = true

  try {
    const response = await fetchSystemRoleDetail(roleId)
    currentDetail.value = response.data
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '角色详情加载失败'))
  }
  finally {
    detailLoading.value = false
  }
}

function handleCreate() {
  modalMode.value = 'create'
  resetForm()
  modalVisible.value = true
}

function handleCloseModal() {
  modalVisible.value = false
  resetForm()
}

async function handleEdit(roleId: number) {
  modalMode.value = 'edit'
  resetForm()
  modalVisible.value = true
  modalLoading.value = true

  try {
    const response = await fetchSystemRoleDetail(roleId)
    const detail = response.data

    editingRoleId.value = detail.id
    form.name = detail.name
    form.status = detail.status
    form.dataScope = detail.data_scope
    form.sort = detail.sort
    form.remark = detail.remark ?? ''
  }
  catch (error) {
    modalVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '角色详情加载失败'))
  }
  finally {
    modalLoading.value = false
  }
}

async function handleSubmit() {
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) {
    return
  }

  submitLoading.value = true

  try {
    if (isCreateMode.value) {
      const payload: CreateRolePayload = {
        code: form.code.trim(),
        name: form.name.trim(),
        status: form.status,
        data_scope: form.dataScope,
        sort: form.sort,
        remark: optionalText(form.remark) ?? undefined,
      }

      await createSystemRole(payload)
      filters.keyword = payload.name
      pagination.page = 1
      ElMessage.success('角色创建成功')
    }
    else if (editingRoleId.value) {
      const payload: UpdateRolePayload = {
        name: optionalText(form.name) ?? undefined,
        status: form.status,
        data_scope: form.dataScope,
        sort: form.sort,
        remark: optionalText(form.remark) ?? undefined,
      }

      await updateSystemRole(editingRoleId.value, payload)
      ElMessage.success('角色更新成功')
    }

    modalVisible.value = false
    await loadRoles()
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, isCreateMode.value ? '角色创建失败' : '角色更新失败'))
  }
  finally {
    submitLoading.value = false
  }
}

async function handleToggleStatus(row: RoleListItem) {
  const nextStatus = row.status === 1 ? 0 : 1
  const actionText = nextStatus === 1 ? '启用' : '停用'

  try {
    await ElMessageBox.confirm(
      `确认${actionText}角色 “${row.name}” 吗？`,
      `${actionText}确认`,
      {
        type: 'warning',
        confirmButtonText: `确认${actionText}`,
        cancelButtonText: '取消',
      },
    )

    await updateSystemRoleStatus(row.id, nextStatus)
    ElMessage.success(`角色已${actionText}`)
    await loadRoles()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, `${actionText}失败`))
  }
}

function handlePermissionConfig(role: RoleListItem | RoleDetailData) {
  ElMessage.info(`“${role.name}” 的权限配置将在后续菜单授权阶段接入。`)
}

function optionalText(value: string) {
  const trimmed = value.trim()
  return trimmed ? trimmed : undefined
}

function formatDateTime(value?: string | null) {
  return value || '未记录'
}

function resolveDataScopeLabel(value: string) {
  return dataScopeOptions.find(option => option.value === value)?.label ?? value
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

onMounted(() => {
  void loadRoles()
})
</script>

<template>
  <PageContent
    eyebrow="System Role"
    title="角色管理"
    description="先完成 RBAC 角色管理的前端原型闭环，包含列表、筛选、详情、新增、编辑、启停和权限配置入口预留。"
  >
    <template #actions>
      <el-button
        type="primary"
        @click="handleCreate"
      >
        <el-icon><Plus /></el-icon>
        <span>新增角色</span>
      </el-button>
      <el-button @click="loadRoles">
        <el-icon><RefreshRight /></el-icon>
        <span>刷新列表</span>
      </el-button>
    </template>

    <section class="role-management__overview">
      <article class="role-management__overview-card role-management__overview-card--total">
        <div class="role-management__overview-head">
          <span>角色总量</span>
          <em>总览</em>
        </div>
        <strong>{{ pagination.total }}</strong>
        <p>当前筛选条件下可维护的角色总数。</p>
      </article>

      <article class="role-management__overview-card role-management__overview-card--success">
        <div class="role-management__overview-head">
          <span>当前页启用角色</span>
          <em>正常</em>
        </div>
        <strong>{{ enabledCount }}</strong>
        <p>用于快速判断当前页角色是否可参与授权。</p>
      </article>

      <article class="role-management__overview-card role-management__overview-card--warning">
        <div class="role-management__overview-head">
          <span>内置角色</span>
          <em>核心</em>
        </div>
        <strong>{{ builtinCount }}</strong>
        <p>内置角色通常与系统关键能力和租户初始化有关。</p>
      </article>
    </section>

    <PageSearch>
      <div class="role-management__search-shell">
        <div class="role-management__section-heading">
          <div class="role-management__section-copy">
            <strong>条件检索</strong>
            <span>先定位角色，再进入状态维护与权限配置。</span>
          </div>

          <div class="role-management__section-side">
            <span
              class="role-management__section-badge"
              :class="{ 'role-management__section-badge--active': hasActiveFilters }"
            >
              {{ hasActiveFilters ? `已启用 ${activeFilterCount} 个筛选条件` : '当前为全部角色' }}
            </span>
          </div>
        </div>

        <el-form
          class="role-management__search-form"
          :model="filters"
          label-position="top"
        >
          <el-form-item label="关键词">
            <el-input
              v-model="filters.keyword"
              placeholder="角色名称 / 编码 / 备注"
              clearable
              @keyup.enter="handleSearch"
            />
          </el-form-item>

          <el-form-item label="状态">
            <el-select
              v-model="filters.status"
              placeholder="全部状态"
              clearable
            >
              <el-option
                v-for="option in statusOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
          </el-form-item>

          <el-form-item label="数据范围">
            <el-select
              v-model="filters.dataScope"
              placeholder="全部范围"
              clearable
            >
              <el-option
                v-for="option in dataScopeOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
          </el-form-item>
        </el-form>

        <div class="role-management__search-tips">
          <span>支持按角色名称、编码和备注快速检索。</span>
          <span>建议优先按状态或数据范围收窄结果，再进入权限配置。</span>
        </div>
      </div>

      <template #actions>
        <div class="role-management__search-actions">
          <p class="role-management__search-summary">
            当前每页 {{ pagination.pageSize }} 条，共 {{ pagination.total }} 条结果
          </p>

          <div class="role-management__search-buttons">
            <el-button @click="handleReset">
              重置
            </el-button>
            <el-button
              type="primary"
              @click="handleSearch"
            >
              <el-icon><Search /></el-icon>
              <span>查询</span>
            </el-button>
          </div>
        </div>
      </template>
    </PageSearch>

    <BaseTable
      :total="pagination.total"
      :page="pagination.page"
      :page-size="pagination.pageSize"
      @update:page="handlePageChange"
      @update:page-size="handlePageSizeChange"
    >
      <template #toolbar>
        <div class="role-management__table-toolbar">
          <div class="role-management__table-meta">
            <strong>角色列表</strong>
            <span>{{ hasActiveFilters ? '当前结果已应用筛选条件，适合直接进入目标角色维护。' : '第一版先承接最常用的查、增、改、启停与权限入口预留。' }}</span>
          </div>

          <div class="role-management__table-pills">
            <span class="role-management__table-pill">第 {{ pagination.page }} / {{ pageCount }} 页</span>
            <span class="role-management__table-pill role-management__table-pill--success">启用 {{ enabledCount }}</span>
            <span class="role-management__table-pill role-management__table-pill--danger">禁用 {{ disabledCount }}</span>
          </div>
        </div>
      </template>

      <el-table
        v-loading="loading"
        :data="roleList"
        row-key="id"
      >
        <el-table-column
          prop="name"
          label="角色名称"
          min-width="150"
        />
        <el-table-column
          prop="code"
          label="角色编码"
          min-width="160"
        />
        <el-table-column
          label="数据范围"
          min-width="140"
        >
          <template #default="{ row }">
            {{ resolveDataScopeLabel(row.data_scope) }}
          </template>
        </el-table-column>
        <el-table-column
          prop="sort"
          label="排序"
          width="90"
        />
        <el-table-column
          label="成员数"
          width="90"
        >
          <template #default="{ row }">
            {{ row.user_count }}
          </template>
        </el-table-column>
        <el-table-column
          label="权限点"
          width="96"
        >
          <template #default="{ row }">
            {{ row.permission_count }}
          </template>
        </el-table-column>
        <el-table-column
          label="状态"
          width="96"
        >
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'danger'">
              {{ row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column
          label="内置"
          width="96"
        >
          <template #default="{ row }">
            <el-tag :type="row.is_builtin ? 'warning' : 'info'">
              {{ row.is_builtin ? '内置' : '自定义' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column
          label="创建时间"
          min-width="168"
        >
          <template #default="{ row }">
            {{ formatDateTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column
          label="操作"
          fixed="right"
          width="320"
        >
          <template #default="{ row }">
            <div class="role-management__row-actions">
              <el-button
                link
                type="primary"
                class="role-management__row-action"
                @click="handleView(row.id)"
              >
                <el-icon><View /></el-icon>
                <span>详情</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="role-management__row-action"
                @click="handleEdit(row.id)"
              >
                <el-icon><EditPen /></el-icon>
                <span>编辑</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="role-management__row-action"
                @click="handlePermissionConfig(row)"
              >
                <el-icon><Lock /></el-icon>
                <span>权限配置</span>
              </el-button>
              <el-button
                link
                :type="row.status === 1 ? 'danger' : 'success'"
                class="role-management__row-action"
                @click="handleToggleStatus(row)"
              >
                <el-icon><CircleCheck /></el-icon>
                <span>{{ row.status === 1 ? '停用' : '启用' }}</span>
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </BaseTable>

    <el-drawer
      v-model="detailVisible"
      title="角色详情"
      size="560px"
    >
      <div
        v-loading="detailLoading"
        class="role-management__detail"
      >
        <template v-if="currentDetail">
          <div class="role-management__detail-hero-shell role-management__panel">
            <div class="role-management__detail-hero">
              <div class="role-management__detail-avatar">
                {{ currentDetail.name.slice(0, 1) }}
              </div>
              <div class="role-management__detail-copy">
                <strong>{{ currentDetail.name }}</strong>
                <span>{{ currentDetail.code }}</span>
              </div>
              <el-tag :type="currentDetail.status === 1 ? 'success' : 'danger'">
                {{ currentDetail.status === 1 ? '启用' : '禁用' }}
              </el-tag>
            </div>

            <div class="role-management__detail-meta">
              <article>
                <span>数据范围</span>
                <strong>{{ resolveDataScopeLabel(currentDetail.data_scope) }}</strong>
              </article>
              <article>
                <span>角色成员</span>
                <strong>{{ currentDetail.user_count }} 人</strong>
              </article>
              <article>
                <span>权限点</span>
                <strong>{{ currentDetail.permission_count }} 项</strong>
              </article>
            </div>
          </div>

          <section class="role-management__detail-section role-management__panel">
            <header class="role-management__detail-section-header">
              <strong>基础资料</strong>
              <span>角色名称、编码、排序和内置属性等基本信息。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="角色名称">
                {{ currentDetail.name }}
              </el-descriptions-item>
              <el-descriptions-item label="角色编码">
                {{ currentDetail.code }}
              </el-descriptions-item>
              <el-descriptions-item label="排序">
                {{ currentDetail.sort }}
              </el-descriptions-item>
              <el-descriptions-item label="角色类型">
                {{ currentDetail.is_builtin ? '内置角色' : '自定义角色' }}
              </el-descriptions-item>
              <el-descriptions-item label="备注">
                {{ currentDetail.remark ?? '无' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="role-management__detail-section role-management__panel">
            <header class="role-management__detail-section-header">
              <strong>授权范围</strong>
              <span>保留数据范围与权限点统计，并预留后续菜单授权入口。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="数据范围">
                {{ resolveDataScopeLabel(currentDetail.data_scope) }}
              </el-descriptions-item>
              <el-descriptions-item label="权限点">
                {{ currentDetail.permissions.length ? currentDetail.permissions.map(item => item.name).join(' / ') : '暂未分配' }}
              </el-descriptions-item>
            </el-descriptions>

            <div class="role-management__detail-actions">
              <el-button
                type="primary"
                plain
                @click="handlePermissionConfig(currentDetail)"
              >
                <el-icon><Lock /></el-icon>
                <span>进入权限配置</span>
              </el-button>
            </div>
          </section>

          <section class="role-management__detail-section role-management__panel">
            <header class="role-management__detail-section-header">
              <strong>审计信息</strong>
              <span>用于确认角色创建时间、最近变更时间和状态。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="创建时间">
                {{ formatDateTime(currentDetail.created_at) }}
              </el-descriptions-item>
              <el-descriptions-item label="更新时间">
                {{ formatDateTime(currentDetail.updated_at) }}
              </el-descriptions-item>
            </el-descriptions>
          </section>
        </template>
      </div>
    </el-drawer>

    <PageModal
      v-model="modalVisible"
      :title="modalTitle"
      width="760px"
      @cancel="resetForm"
    >
      <div
        v-loading="modalLoading"
        class="role-management__modal-body"
      >
        <el-form
          ref="formRef"
          :model="form"
          :rules="formRules"
          label-position="top"
          class="role-management__form"
        >
          <section class="role-management__form-section role-management__panel">
            <strong>基础信息</strong>
            <span>维护角色标识、展示名称和角色排序。</span>

            <div class="role-management__form-grid">
              <el-form-item
                v-if="isCreateMode"
                label="角色编码"
                prop="code"
              >
                <el-input
                  v-model="form.code"
                  placeholder="请输入角色编码"
                />
              </el-form-item>

              <el-form-item
                v-else
                label="角色编码"
                class="role-management__form-span-2"
              >
                <el-input
                  :model-value="currentEditingCode"
                  readonly
                />
              </el-form-item>

              <el-form-item
                label="角色名称"
                prop="name"
              >
                <el-input
                  v-model="form.name"
                  placeholder="请输入角色名称"
                />
              </el-form-item>

              <el-form-item label="排序">
                <el-input-number
                  v-model="form.sort"
                  :min="0"
                  :max="999"
                  controls-position="right"
                  class="role-management__number-input"
                />
              </el-form-item>
            </div>
          </section>

          <section class="role-management__form-section role-management__panel">
            <strong>授权范围</strong>
            <span>设置角色数据范围，并预留后续菜单与权限绑定能力。</span>

            <div class="role-management__form-grid">
              <el-form-item
                label="数据范围"
                prop="dataScope"
              >
                <el-select v-model="form.dataScope">
                  <el-option
                    v-for="option in dataScopeOptions"
                    :key="option.value"
                    :label="option.label"
                    :value="option.value"
                  />
                </el-select>
              </el-form-item>

              <el-form-item label="状态">
                <el-radio-group v-model="form.status">
                  <el-radio
                    v-for="option in statusOptions"
                    :key="option.value"
                    :value="option.value"
                  >
                    {{ option.label }}
                  </el-radio>
                </el-radio-group>
              </el-form-item>

              <el-form-item
                label="备注"
                class="role-management__form-span-2"
              >
                <el-input
                  v-model="form.remark"
                  type="textarea"
                  :rows="3"
                  placeholder="请输入备注"
                />
              </el-form-item>
            </div>
          </section>
        </el-form>
      </div>

      <template #footer>
        <el-button @click="handleCloseModal">
          取消
        </el-button>
        <el-button
          type="primary"
          :loading="submitLoading"
          @click="handleSubmit"
        >
          {{ isCreateMode ? '创建角色' : '保存修改' }}
        </el-button>
      </template>
    </PageModal>
  </PageContent>
</template>

<style scoped>
.role-management__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.role-management__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  position: relative;
  overflow: hidden;
}

.role-management__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
  background: var(--app-primary);
}

.role-management__overview-card--success::before {
  background: var(--app-success);
}

.role-management__overview-card--warning::before {
  background: var(--app-warning);
}

.role-management__overview-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.role-management__overview-head em {
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.role-management__overview-card--success .role-management__overview-head em {
  background: color-mix(in srgb, var(--app-success) 10%, white);
  color: var(--app-success);
}

.role-management__overview-card--warning .role-management__overview-head em {
  background: color-mix(in srgb, var(--app-warning) 14%, white);
  color: #b45309;
}

.role-management__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.role-management__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.role-management__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.role-management__search-form {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.role-management__search-shell {
  display: grid;
  gap: 14px;
}

.role-management__section-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.role-management__section-copy {
  display: grid;
  gap: 4px;
}

.role-management__section-copy strong {
  color: #111827;
  font-size: 15px;
}

.role-management__section-copy span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-management__section-side {
  display: flex;
  align-items: center;
}

.role-management__section-badge {
  display: inline-flex;
  min-height: 30px;
  align-items: center;
  padding: 0 12px;
  border: 1px solid var(--app-border);
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 12px;
  font-weight: 600;
}

.role-management__section-badge--active {
  border-color: color-mix(in srgb, var(--app-primary) 18%, white);
  background: var(--app-primary-soft);
  color: var(--app-primary);
}

.role-management__search-tips {
  display: flex;
  gap: 10px 18px;
  flex-wrap: wrap;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-management__search-actions {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.role-management__search-summary {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-management__search-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.role-management__search-form :deep(.el-form-item) {
  margin-bottom: 0;
}

.role-management__table-toolbar {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.role-management__table-meta {
  display: grid;
  gap: 4px;
}

.role-management__table-meta strong {
  color: #111827;
  font-size: 15px;
}

.role-management__table-meta span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.role-management__table-pills {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.role-management__table-pill {
  display: inline-flex;
  align-items: center;
  min-height: 30px;
  padding: 0 12px;
  border-radius: 999px;
  background: #f3f6fb;
  color: #425466;
  font-size: 12px;
  font-weight: 600;
}

.role-management__table-pill--success {
  background: #edf9f1;
  color: #1f8f55;
}

.role-management__table-pill--danger {
  background: #fdf0f0;
  color: #c24141;
}

.role-management__row-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: nowrap;
}

.role-management__row-action {
  margin-left: 0;
  justify-content: flex-start;
  padding: 2px 0;
  flex-shrink: 0;
}

.role-management__panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-md);
  background: var(--app-surface-strong);
}

.role-management__detail {
  display: grid;
  gap: 16px;
  min-height: 240px;
}

.role-management__detail-hero-shell {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.role-management__detail-hero {
  display: flex;
  align-items: center;
  gap: 12px;
}

.role-management__detail-avatar {
  display: inline-flex;
  width: 44px;
  height: 44px;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: #7183d8;
  color: #fff;
  font-size: 16px;
  font-weight: 700;
}

.role-management__detail-copy {
  display: grid;
  gap: 4px;
  flex: 1;
}

.role-management__detail-copy strong {
  color: #111827;
  font-size: 18px;
}

.role-management__detail-copy span {
  color: var(--app-text-soft);
  font-size: 13px;
}

.role-management__detail-meta {
  display: grid;
  gap: 10px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.role-management__detail-meta article {
  display: grid;
  gap: 6px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.role-management__detail-meta article span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.role-management__detail-meta article strong {
  color: #111827;
  font-size: 14px;
  line-height: 1.6;
}

.role-management__detail-section {
  display: grid;
  gap: 10px;
  padding: 18px;
}

.role-management__detail-section-header {
  display: grid;
  gap: 4px;
}

.role-management__detail-section-header strong {
  color: #111827;
  font-size: 14px;
}

.role-management__detail-section-header span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-management__detail-actions {
  display: flex;
  justify-content: flex-end;
}

.role-management__modal-body {
  min-height: 220px;
}

.role-management__form {
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr;
}

.role-management__form-section {
  display: grid;
  gap: 14px;
  padding: 18px;
}

.role-management__form-section strong {
  color: #111827;
  font-size: 14px;
}

.role-management__form-section span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.role-management__form-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.role-management__form-span-2 {
  grid-column: 1 / -1;
}

.role-management__number-input {
  width: 100%;
}

.role-management__form :deep(.el-form-item) {
  margin-bottom: 0;
}

@media (max-width: 1080px) {
  .role-management__overview,
  .role-management__search-form,
  .role-management__form-grid {
    grid-template-columns: 1fr;
  }

  .role-management__search-actions {
    align-items: flex-start;
  }

  .role-management__detail-meta {
    grid-template-columns: 1fr;
  }

  .role-management__form-span-2 {
    grid-column: auto;
  }
}
</style>
