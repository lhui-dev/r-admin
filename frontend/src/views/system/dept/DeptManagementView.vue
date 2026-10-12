<script setup lang="ts">
import {
  CircleCheck,
  Delete,
  EditPen,
  FolderAdd,
  OfficeBuilding,
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
  createSystemDept,
  deleteSystemDept,
  fetchSystemDeptDetail,
  fetchSystemDeptTree,
  updateSystemDept,
  updateSystemDeptStatus,
  type CreateDeptPayload,
  type DeptTreeItem,
  type UpdateDeptPayload,
} from '@/api/system-dept'

type DeptFormModel = {
  parentId: number
  deptName: string
  deptCode: string
  leaderUserId?: number
  sortNo: number
  status: number
  remark: string
}

type ParentOption = {
  id: number
  label: string
  disabled: boolean
}

const statusOptions = [
  { label: '启用', value: 1 },
  { label: '禁用', value: 0 },
]

const filters = reactive({
  keyword: '',
  status: undefined as number | undefined,
})

const deptTree = ref<DeptTreeItem[]>([])
const loading = ref(false)
const detailVisible = ref(false)
const detailLoading = ref(false)
const currentDetail = ref<DeptTreeItem | null>(null)
const modalVisible = ref(false)
const modalLoading = ref(false)
const submitLoading = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingDeptId = ref<number | null>(null)
const formRef = ref<FormInstance>()

const form = reactive<DeptFormModel>(createDefaultForm())

const formRules: FormRules<DeptFormModel> = {
  deptName: [
    { required: true, message: '请输入部门名称', trigger: 'blur' },
  ],
  deptCode: [
    {
      validator: (_rule, value, callback) => {
        const text = String(value ?? '').trim()
        if (text && !/^[A-Z0-9_-]+$/i.test(text)) {
          callback(new Error('部门编码仅支持字母、数字、下划线和中划线'))
          return
        }

        callback()
      },
      trigger: 'blur',
    },
  ],
}

const isCreateMode = computed(() => modalMode.value === 'create')
const modalTitle = computed(() => (isCreateMode.value ? '新增部门' : '编辑部门'))
const flatDepts = computed(() => flattenDeptTree(deptTree.value))
const enabledCount = computed(() => flatDepts.value.filter(item => item.status === 1).length)
const disabledCount = computed(() => flatDepts.value.filter(item => item.status === 0).length)
const rootDeptCount = computed(() => deptTree.value.length)
const activeFilterCount = computed(() => [filters.keyword, filters.status].filter(value => value !== '' && value !== undefined).length)
const hasActiveFilters = computed(() => activeFilterCount.value > 0)
const parentOptions = computed<ParentOption[]>(() => {
  const blockedIds = editingDeptId.value ? collectDescendantIds(editingDeptId.value, deptTree.value) : new Set<number>()
  if (editingDeptId.value) {
    blockedIds.add(editingDeptId.value)
  }

  return [
    { id: 0, label: '根部门', disabled: false },
    ...flatDepts.value.map(item => ({
      id: item.id,
      label: `${'　'.repeat(item.level)}${item.dept_name}`,
      disabled: blockedIds.has(item.id),
    })),
  ]
})

function createDefaultForm(): DeptFormModel {
  return {
    parentId: 0,
    deptName: '',
    deptCode: '',
    leaderUserId: undefined,
    sortNo: 100,
    status: 1,
    remark: '',
  }
}

function resetForm() {
  Object.assign(form, createDefaultForm())
  editingDeptId.value = null
  formRef.value?.clearValidate()
}

async function loadDepts() {
  loading.value = true

  try {
    const response = await fetchSystemDeptTree({
      keyword: filters.keyword || undefined,
      status: filters.status,
    })

    deptTree.value = response.data.items
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '部门树加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleSearch() {
  void loadDepts()
}

function handleReset() {
  filters.keyword = ''
  filters.status = undefined
  void loadDepts()
}

async function handleView(deptId: number) {
  detailVisible.value = true
  detailLoading.value = true

  try {
    const response = await fetchSystemDeptDetail(deptId)
    currentDetail.value = response.data
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '部门详情加载失败'))
  }
  finally {
    detailLoading.value = false
  }
}

function handleCreate(parentId = 0) {
  modalMode.value = 'create'
  resetForm()
  form.parentId = parentId
  modalVisible.value = true
}

function handleCloseModal() {
  modalVisible.value = false
  resetForm()
}

async function handleEdit(deptId: number) {
  modalMode.value = 'edit'
  resetForm()
  modalVisible.value = true
  modalLoading.value = true

  try {
    const response = await fetchSystemDeptDetail(deptId)
    const detail = response.data

    editingDeptId.value = detail.id
    form.parentId = detail.parent_id
    form.deptName = detail.dept_name
    form.deptCode = detail.dept_code ?? ''
    form.leaderUserId = detail.leader_user_id ?? undefined
    form.sortNo = detail.sort_no
    form.status = detail.status
    form.remark = detail.remark ?? ''
  }
  catch (error) {
    modalVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '部门详情加载失败'))
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

  const targetDeptId = editingDeptId.value
  submitLoading.value = true

  try {
    if (isCreateMode.value) {
      await createSystemDept(buildCreatePayload())
      ElMessage.success('部门创建成功')
    }
    else if (targetDeptId) {
      await updateSystemDept(targetDeptId, buildUpdatePayload())
      ElMessage.success('部门更新成功')
    }

    modalVisible.value = false
    await loadDepts()
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, isCreateMode.value ? '部门创建失败' : '部门更新失败'))
  }
  finally {
    submitLoading.value = false
  }
}

async function handleToggleStatus(row: DeptTreeItem) {
  const nextStatus = row.status === 1 ? 0 : 1
  const actionText = nextStatus === 1 ? '启用' : '停用'

  try {
    await ElMessageBox.confirm(
      `确认${actionText}部门 “${row.dept_name}” 吗？`,
      `${actionText}确认`,
      {
        type: 'warning',
        confirmButtonText: `确认${actionText}`,
        cancelButtonText: '取消',
      },
    )

    await updateSystemDeptStatus(row.id, nextStatus)
    ElMessage.success(`部门已${actionText}`)
    await loadDepts()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, `${actionText}失败`))
  }
}

async function handleDelete(row: DeptTreeItem) {
  try {
    await ElMessageBox.confirm(
      `确认删除部门 “${row.dept_name}” 吗？如存在子部门或用户引用，后端会拒绝删除。`,
      '删除确认',
      {
        type: 'warning',
        confirmButtonText: '确认删除',
        cancelButtonText: '取消',
      },
    )

    await deleteSystemDept(row.id)
    ElMessage.success('部门已删除')
    await loadDepts()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, '部门删除失败'))
  }
}

function buildCreatePayload(): CreateDeptPayload {
  return {
    parent_id: form.parentId,
    dept_name: form.deptName.trim(),
    dept_code: optionalText(form.deptCode),
    leader_user_id: form.leaderUserId,
    sort_no: form.sortNo,
    status: form.status,
    remark: optionalText(form.remark),
  }
}

function buildUpdatePayload(): UpdateDeptPayload {
  return {
    parent_id: form.parentId,
    dept_name: form.deptName.trim(),
    dept_code: form.deptCode.trim(),
    leader_user_id: form.leaderUserId ?? null,
    sort_no: form.sortNo,
    status: form.status,
    remark: form.remark.trim(),
  }
}

function optionalText(value: string) {
  const trimmed = value.trim()
  return trimmed ? trimmed : undefined
}

function flattenDeptTree(items: DeptTreeItem[], level = 0): Array<DeptTreeItem & { level: number }> {
  return items.flatMap(item => [
    { ...item, level },
    ...flattenDeptTree(item.children, level + 1),
  ])
}

function collectDescendantIds(targetId: number, items: DeptTreeItem[]) {
  const ids = new Set<number>()

  function visit(nodes: DeptTreeItem[]) {
    nodes.forEach((node) => {
      if (ids.has(node.parent_id) || node.parent_id === targetId) {
        ids.add(node.id)
      }

      visit(node.children)
    })
  }

  visit(items)
  return ids
}

function resolveStatusType(status: number) {
  return status === 1 ? 'success' : 'info'
}

function resolveStatusLabel(status: number) {
  return status === 1 ? '启用' : '禁用'
}

function resolveErrorMessage(error: unknown, fallback: string) {
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message?: unknown }).message ?? fallback)
  }

  return fallback
}

onMounted(() => {
  void loadDepts()
})
</script>

<template>
  <PageContent
    eyebrow="Organization"
    title="部门管理"
    description="维护组织部门树，为用户归属、数据权限和后续多租户扩展提供基础组织结构。"
  >
    <template #actions>
      <el-button
        :icon="RefreshRight"
        :disabled="loading"
        @click="handleReset"
      >
        重置
      </el-button>
      <el-button
        type="primary"
        :icon="Plus"
        @click="handleCreate()"
      >
        新增部门
      </el-button>
    </template>

    <section class="dept-management__overview">
      <article class="dept-management__overview-card dept-management__overview-card--primary">
        <div class="dept-management__overview-head">
          <span>部门总数</span>
          <em>总览</em>
        </div>
        <strong>{{ flatDepts.length }}</strong>
        <p>当前组织树中已维护的有效部门节点。</p>
      </article>
      <article class="dept-management__overview-card dept-management__overview-card--success">
        <div class="dept-management__overview-head">
          <span>启用部门</span>
          <em>可用</em>
        </div>
        <strong>{{ enabledCount }}</strong>
        <p>可被用户归属和数据权限引用。</p>
      </article>
      <article class="dept-management__overview-card dept-management__overview-card--warning">
        <div class="dept-management__overview-head">
          <span>根级部门</span>
          <em>结构</em>
        </div>
        <strong>{{ rootDeptCount }}</strong>
        <p>组织架构第一层部门数量。</p>
      </article>
      <article class="dept-management__overview-card dept-management__overview-card--danger">
        <div class="dept-management__overview-head">
          <span>禁用部门</span>
          <em>关注</em>
        </div>
        <strong>{{ disabledCount }}</strong>
        <p>暂不参与业务分配的部门。</p>
      </article>
    </section>

    <PageSearch>
      <el-form
        class="dept-management__search-form"
        label-position="top"
      >
        <el-form-item label="关键词">
          <el-input
            v-model="filters.keyword"
            clearable
            placeholder="搜索部门名称、编码或负责人"
            @keyup.enter="handleSearch"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select
            v-model="filters.status"
            clearable
            placeholder="全部状态"
          >
            <el-option
              v-for="item in statusOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>
      </el-form>

      <template #actions>
        <span
          v-if="hasActiveFilters"
          class="dept-management__filter-count"
        >
          已启用 {{ activeFilterCount }} 个筛选条件
        </span>
        <el-button
          :icon="RefreshRight"
          @click="handleReset"
        >
          清空
        </el-button>
        <el-button
          type="primary"
          :icon="Search"
          :loading="loading"
          @click="handleSearch"
        >
          查询
        </el-button>
      </template>
    </PageSearch>

    <BaseTable
      class="dept-management__table-shell"
      :total="flatDepts.length"
      :page="1"
      :page-size="flatDepts.length || 10"
      :show-pagination="false"
    >
      <template #toolbar>
        <div class="dept-management__table-toolbar">
          <div class="dept-management__table-meta">
            <strong>组织部门树</strong>
            <span>{{ hasActiveFilters ? '当前结果已应用筛选条件，适合直接维护目标部门。' : '维护组织架构、上下级关系和部门负责人。' }}</span>
          </div>

          <div class="dept-management__table-pills">
            <span class="dept-management__table-pill">节点 {{ flatDepts.length }}</span>
            <span class="dept-management__table-pill dept-management__table-pill--success">启用 {{ enabledCount }}</span>
            <span class="dept-management__table-pill dept-management__table-pill--danger">禁用 {{ disabledCount }}</span>
          </div>
        </div>
      </template>

      <el-table
        v-loading="loading"
        :data="deptTree"
        row-key="id"
        default-expand-all
        class="dept-management__table"
        :tree-props="{ children: 'children' }"
      >
        <el-table-column
          label="部门名称"
          min-width="220"
        >
          <template #default="{ row }">
            <div class="dept-management__name-cell">
              <el-icon><OfficeBuilding /></el-icon>
              <div>
                <strong>{{ row.dept_name }}</strong>
                <span>{{ row.dept_code || '未设置编码' }}</span>
              </div>
            </div>
          </template>
        </el-table-column>

        <el-table-column
          label="负责人"
          min-width="140"
        >
          <template #default="{ row }">
            {{ row.leader_name || row.leader_user_id || '未设置' }}
          </template>
        </el-table-column>

        <el-table-column
          label="排序"
          prop="sort_no"
          width="90"
        />

        <el-table-column
          label="状态"
          width="100"
        >
          <template #default="{ row }">
            <el-tag
              round
              :type="resolveStatusType(row.status)"
            >
              {{ resolveStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column
          label="更新时间"
          prop="updated_at"
          min-width="180"
        />

        <el-table-column
          label="操作"
          fixed="right"
          width="360"
        >
          <template #default="{ row }">
            <div class="dept-management__row-actions">
              <el-button
                link
                type="primary"
                class="dept-management__row-action"
                @click="handleView(row.id)"
              >
                <el-icon><View /></el-icon>
                <span>详情</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="dept-management__row-action"
                @click="handleCreate(row.id)"
              >
                <el-icon><FolderAdd /></el-icon>
                <span>新增下级</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="dept-management__row-action"
                @click="handleEdit(row.id)"
              >
                <el-icon><EditPen /></el-icon>
                <span>编辑</span>
              </el-button>
              <el-button
                link
                :type="row.status === 1 ? 'danger' : 'success'"
                class="dept-management__row-action"
                @click="handleToggleStatus(row)"
              >
                <el-icon><CircleCheck /></el-icon>
                <span>{{ row.status === 1 ? '停用' : '启用' }}</span>
              </el-button>
              <el-button
                link
                type="danger"
                class="dept-management__row-action"
                @click="handleDelete(row)"
              >
                <el-icon><Delete /></el-icon>
                <span>删除</span>
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </BaseTable>

    <el-drawer
      v-model="detailVisible"
      title="部门详情"
      size="420px"
    >
      <el-skeleton
        v-if="detailLoading"
        :rows="8"
        animated
      />
      <el-descriptions
        v-else-if="currentDetail"
        :column="1"
        border
      >
        <el-descriptions-item label="部门名称">
          {{ currentDetail.dept_name }}
        </el-descriptions-item>
        <el-descriptions-item label="部门编码">
          {{ currentDetail.dept_code || '未设置' }}
        </el-descriptions-item>
        <el-descriptions-item label="上级部门 ID">
          {{ currentDetail.parent_id || '根部门' }}
        </el-descriptions-item>
        <el-descriptions-item label="负责人">
          {{ currentDetail.leader_name || currentDetail.leader_user_id || '未设置' }}
        </el-descriptions-item>
        <el-descriptions-item label="排序">
          {{ currentDetail.sort_no }}
        </el-descriptions-item>
        <el-descriptions-item label="状态">
          {{ resolveStatusLabel(currentDetail.status) }}
        </el-descriptions-item>
        <el-descriptions-item label="备注">
          {{ currentDetail.remark || '无' }}
        </el-descriptions-item>
        <el-descriptions-item label="创建时间">
          {{ currentDetail.created_at }}
        </el-descriptions-item>
        <el-descriptions-item label="更新时间">
          {{ currentDetail.updated_at }}
        </el-descriptions-item>
      </el-descriptions>
    </el-drawer>

    <PageModal
      v-model="modalVisible"
      :title="modalTitle"
      width="680px"
      @cancel="handleCloseModal"
    >
      <el-skeleton
        v-if="modalLoading"
        :rows="8"
        animated
      />
      <el-form
        v-else
        ref="formRef"
        :model="form"
        :rules="formRules"
        label-position="top"
        class="dept-management__form"
      >
        <el-form-item
          label="上级部门"
          prop="parentId"
        >
          <el-select
            v-model="form.parentId"
            filterable
            placeholder="请选择上级部门"
          >
            <el-option
              v-for="item in parentOptions"
              :key="item.id"
              :label="item.label"
              :value="item.id"
              :disabled="item.disabled"
            />
          </el-select>
        </el-form-item>

        <el-form-item
          label="部门名称"
          prop="deptName"
        >
          <el-input
            v-model="form.deptName"
            maxlength="64"
            show-word-limit
            placeholder="请输入部门名称"
          />
        </el-form-item>

        <el-form-item
          label="部门编码"
          prop="deptCode"
        >
          <el-input
            v-model="form.deptCode"
            maxlength="64"
            show-word-limit
            placeholder="例如 TECH_CENTER"
          />
        </el-form-item>

        <el-form-item label="负责人用户 ID">
          <el-input-number
            v-model="form.leaderUserId"
            :min="1"
            :controls="false"
            placeholder="后续会替换为用户选择器"
          />
        </el-form-item>

        <div class="dept-management__form-grid">
          <el-form-item label="排序">
            <el-input-number
              v-model="form.sortNo"
              :min="0"
              :max="9999"
            />
          </el-form-item>
          <el-form-item label="状态">
            <el-radio-group v-model="form.status">
              <el-radio-button
                v-for="item in statusOptions"
                :key="item.value"
                :value="item.value"
              >
                {{ item.label }}
              </el-radio-button>
            </el-radio-group>
          </el-form-item>
        </div>

        <el-form-item label="备注">
          <el-input
            v-model="form.remark"
            type="textarea"
            :rows="3"
            maxlength="500"
            show-word-limit
            placeholder="补充部门职责或维护说明"
          />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="handleCloseModal">
          取消
        </el-button>
        <el-button
          type="primary"
          :loading="submitLoading"
          @click="handleSubmit"
        >
          <el-icon><CircleCheck /></el-icon>
          <span>{{ isCreateMode ? '创建部门' : '保存修改' }}</span>
        </el-button>
      </template>
    </PageModal>
  </PageContent>
</template>

<style scoped>
.dept-management__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.dept-management__overview-card {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  box-shadow: var(--app-shadow-soft);
  position: relative;
  overflow: hidden;
}

.dept-management__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
}

.dept-management__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
  background: var(--app-primary);
}

.dept-management__overview-card--primary {
  background:
    radial-gradient(circle at 100% 0%, rgba(31, 122, 255, 0.14), transparent 34%),
    var(--app-surface-strong);
}

.dept-management__overview-card--success::before {
  background: var(--app-success);
}

.dept-management__overview-card--warning::before {
  background: var(--app-warning);
}

.dept-management__overview-card--danger::before {
  background: #c24141;
}

.dept-management__overview-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.dept-management__overview-head em {
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.dept-management__overview-card--success .dept-management__overview-head em {
  background: color-mix(in srgb, var(--app-success) 10%, white);
  color: var(--app-success);
}

.dept-management__overview-card--warning .dept-management__overview-head em {
  background: color-mix(in srgb, var(--app-warning) 14%, white);
  color: #b45309;
}

.dept-management__overview-card--danger .dept-management__overview-head em {
  background: #fdf0f0;
  color: #c24141;
}

.dept-management__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.dept-management__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.dept-management__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.dept-management__search-form {
  display: grid;
  gap: 12px;
  grid-template-columns: minmax(220px, 1fr) 180px;
}

.dept-management__filter-count {
  align-self: center;
  color: var(--app-text-soft);
  font-size: 12px;
}

.dept-management__table-toolbar {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.dept-management__table-meta {
  display: grid;
  gap: 4px;
}

.dept-management__table-meta strong {
  color: #111827;
  font-size: 15px;
}

.dept-management__table-meta span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.dept-management__table-pills {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.dept-management__table-pill {
  display: inline-flex;
  align-items: center;
  min-height: 30px;
  padding: 0 12px;
  border-radius: 999px;
  background: #f3f6fb;
  color: #425466;
  font-size: 12px;
}

.dept-management__table-pill--success {
  background: #edf9f1;
  color: #1f8f55;
}

.dept-management__table-pill--danger {
  background: #fdf0f0;
  color: #c24141;
}

.dept-management__table {
  width: 100%;
}

.dept-management__name-cell {
  display: inline-flex;
  align-items: center;
  gap: 10px;
}

.dept-management__name-cell .el-icon {
  width: 30px;
  height: 30px;
  border-radius: 10px;
  background: color-mix(in srgb, var(--app-primary) 9%, white);
  color: var(--app-primary);
}

.dept-management__name-cell div {
  display: grid;
  gap: 2px;
}

.dept-management__name-cell strong {
  color: #111827;
  font-size: 14px;
}

.dept-management__name-cell span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.dept-management__row-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: nowrap;
  white-space: nowrap;
}

.dept-management__row-action {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  margin-left: 0;
  justify-content: flex-start;
  padding: 2px 0;
  flex-shrink: 0;
}

.dept-management__form {
  display: grid;
  gap: 4px;
}

.dept-management__form-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

:deep(.dept-management__table .el-table__cell) {
  vertical-align: middle;
}

@media (max-width: 1180px) {
  .dept-management__overview {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 760px) {
  .dept-management__overview,
  .dept-management__search-form,
  .dept-management__form-grid {
    grid-template-columns: 1fr;
  }
}
</style>
