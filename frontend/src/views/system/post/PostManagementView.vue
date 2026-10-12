<script setup lang="ts">
import {
  CircleCheck,
  Delete,
  EditPen,
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
  createSystemPost,
  deleteSystemPost,
  fetchSystemPostDetail,
  fetchSystemPosts,
  updateSystemPost,
  updateSystemPostStatus,
  type CreatePostPayload,
  type PostDetailData,
  type PostListItem,
  type UpdatePostPayload,
} from '@/api/system-post'

type PostFormModel = {
  code: string
  name: string
  sort: number
  status: number
  remark: string
}

const statusOptions = [
  { label: '启用', value: 1 },
  { label: '禁用', value: 0 },
]

const filters = reactive({
  keyword: '',
  status: undefined as number | undefined,
})

const pagination = reactive({
  page: 1,
  pageSize: 10,
  total: 0,
})

const postList = ref<PostListItem[]>([])
const loading = ref(false)
const detailVisible = ref(false)
const detailLoading = ref(false)
const currentDetail = ref<PostDetailData | null>(null)
const modalVisible = ref(false)
const modalLoading = ref(false)
const submitLoading = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingPostId = ref<number | null>(null)
const formRef = ref<FormInstance>()

const form = reactive<PostFormModel>(createDefaultForm())

const formRules: FormRules<PostFormModel> = {
  code: [
    { required: true, message: '请输入岗位编码', trigger: 'blur' },
  ],
  name: [
    { required: true, message: '请输入岗位名称', trigger: 'blur' },
  ],
}

const isCreateMode = computed(() => modalMode.value === 'create')
const modalTitle = computed(() => (isCreateMode.value ? '新增岗位' : '编辑岗位'))
const enabledCount = computed(() => postList.value.filter(item => item.status === 1).length)
const disabledCount = computed(() => postList.value.filter(item => item.status === 0).length)
const assignedUserCount = computed(() => postList.value.reduce((total, item) => total + item.user_count, 0))
const pageCount = computed(() => Math.max(1, Math.ceil(pagination.total / pagination.pageSize)))
const activeFilterCount = computed(() => [filters.keyword, filters.status].filter(value => value !== '' && value !== undefined).length)
const hasActiveFilters = computed(() => activeFilterCount.value > 0)
const currentEditingCode = computed(() => {
  if (!editingPostId.value) {
    return ''
  }

  return postList.value.find(item => item.id === editingPostId.value)?.code ?? ''
})

function createDefaultForm(): PostFormModel {
  return {
    code: '',
    name: '',
    sort: 100,
    status: 1,
    remark: '',
  }
}

function resetForm() {
  Object.assign(form, createDefaultForm())
  editingPostId.value = null
  formRef.value?.clearValidate()
}

async function loadPosts() {
  loading.value = true

  try {
    const response = await fetchSystemPosts({
      page: pagination.page,
      page_size: pagination.pageSize,
      keyword: filters.keyword || undefined,
      status: filters.status,
    })

    postList.value = response.data.items
    pagination.total = response.data.pagination.total
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '岗位列表加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleSearch() {
  pagination.page = 1
  void loadPosts()
}

function handleReset() {
  filters.keyword = ''
  filters.status = undefined
  pagination.page = 1
  void loadPosts()
}

function handlePageChange(page: number) {
  pagination.page = page
  void loadPosts()
}

function handlePageSizeChange(pageSize: number) {
  pagination.page = 1
  pagination.pageSize = pageSize
  void loadPosts()
}

async function handleView(postId: number) {
  detailVisible.value = true
  detailLoading.value = true

  try {
    const response = await fetchSystemPostDetail(postId)
    currentDetail.value = response.data
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '岗位详情加载失败'))
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

async function handleEdit(postId: number) {
  modalMode.value = 'edit'
  resetForm()
  modalVisible.value = true
  modalLoading.value = true

  try {
    const response = await fetchSystemPostDetail(postId)
    const detail = response.data

    editingPostId.value = detail.id
    form.name = detail.name
    form.sort = detail.sort
    form.status = detail.status
    form.remark = detail.remark ?? ''
  }
  catch (error) {
    modalVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '岗位详情加载失败'))
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
      const payload: CreatePostPayload = {
        code: form.code.trim(),
        name: form.name.trim(),
        sort: form.sort,
        status: form.status,
        remark: optionalText(form.remark) ?? undefined,
      }

      await createSystemPost(payload)
      filters.keyword = payload.name
      pagination.page = 1
      ElMessage.success('岗位创建成功')
    }
    else if (editingPostId.value) {
      const payload: UpdatePostPayload = {
        name: optionalText(form.name) ?? undefined,
        sort: form.sort,
        status: form.status,
        remark: optionalText(form.remark) ?? undefined,
      }

      await updateSystemPost(editingPostId.value, payload)
      ElMessage.success('岗位更新成功')
    }

    modalVisible.value = false
    await loadPosts()
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, isCreateMode.value ? '岗位创建失败' : '岗位更新失败'))
  }
  finally {
    submitLoading.value = false
  }
}

async function handleToggleStatus(row: PostListItem) {
  const nextStatus = row.status === 1 ? 0 : 1
  const actionText = nextStatus === 1 ? '启用' : '停用'

  try {
    await ElMessageBox.confirm(
      `确认${actionText}岗位 “${row.name}” 吗？`,
      `${actionText}确认`,
      {
        type: 'warning',
        confirmButtonText: `确认${actionText}`,
        cancelButtonText: '取消',
      },
    )

    await updateSystemPostStatus(row.id, nextStatus)
    ElMessage.success(`岗位已${actionText}`)
    await loadPosts()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, `${actionText}失败`))
  }
}

async function handleDelete(row: PostListItem) {
  try {
    await ElMessageBox.confirm(
      `确认删除岗位 “${row.name}” 吗？如该岗位已绑定用户，后端会拒绝删除。`,
      '删除确认',
      {
        type: 'warning',
        confirmButtonText: '确认删除',
        cancelButtonText: '取消',
      },
    )

    await deleteSystemPost(row.id)
    ElMessage.success('岗位已删除')
    await loadPosts()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, '岗位删除失败'))
  }
}

function optionalText(value: string) {
  const trimmed = value.trim()
  return trimmed ? trimmed : undefined
}

function formatDateTime(value?: string | null) {
  return value || '未记录'
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
  void loadPosts()
})
</script>

<template>
  <PageContent
    eyebrow="System Post"
    title="岗位管理"
    description="维护组织岗位定义、岗位编码、排序和启停状态，为用户归属、岗位筛选和后续组织权限扩展提供基础数据。"
  >
    <template #actions>
      <el-button
        type="primary"
        @click="handleCreate"
      >
        <el-icon><Plus /></el-icon>
        <span>新增岗位</span>
      </el-button>
      <el-button @click="loadPosts">
        <el-icon><RefreshRight /></el-icon>
        <span>刷新列表</span>
      </el-button>
    </template>

    <section class="post-management__overview">
      <article class="post-management__overview-card post-management__overview-card--total">
        <div class="post-management__overview-head">
          <span>岗位总量</span>
          <em>总览</em>
        </div>
        <strong>{{ pagination.total }}</strong>
        <p>当前筛选条件下可维护的岗位总数。</p>
      </article>

      <article class="post-management__overview-card post-management__overview-card--success">
        <div class="post-management__overview-head">
          <span>当前页启用岗位</span>
          <em>正常</em>
        </div>
        <strong>{{ enabledCount }}</strong>
        <p>启用岗位可作为用户组织归属和业务分工标签。</p>
      </article>

      <article class="post-management__overview-card post-management__overview-card--warning">
        <div class="post-management__overview-head">
          <span>当前页绑定用户</span>
          <em>关联</em>
        </div>
        <strong>{{ assignedUserCount }}</strong>
        <p>用于判断岗位是否已被用户引用，删除前需要重点确认。</p>
      </article>
    </section>

    <PageSearch>
      <div class="post-management__search-shell">
        <div class="post-management__section-heading">
          <div class="post-management__section-copy">
            <strong>条件检索</strong>
            <span>按岗位名称、编码或备注快速定位目标岗位。</span>
          </div>

          <div class="post-management__section-side">
            <span
              class="post-management__section-badge"
              :class="{ 'post-management__section-badge--active': hasActiveFilters }"
            >
              {{ hasActiveFilters ? `已启用 ${activeFilterCount} 个筛选条件` : '当前为全部岗位' }}
            </span>
          </div>
        </div>

        <el-form
          class="post-management__search-form"
          :model="filters"
          label-position="top"
        >
          <el-form-item label="关键词">
            <el-input
              v-model="filters.keyword"
              placeholder="岗位名称 / 编码 / 备注"
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
        </el-form>

        <div class="post-management__search-tips">
          <span>建议岗位编码保持稳定，避免影响用户岗位关系和后续权限策略。</span>
          <span>已绑定用户的岗位删除会由后端拦截，优先使用停用。</span>
        </div>
      </div>

      <template #actions>
        <div class="post-management__search-actions">
          <p class="post-management__search-summary">
            当前每页 {{ pagination.pageSize }} 条，共 {{ pagination.total }} 条结果
          </p>

          <div class="post-management__search-buttons">
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
        <div class="post-management__table-toolbar">
          <div class="post-management__table-meta">
            <strong>岗位列表</strong>
            <span>{{ hasActiveFilters ? '当前结果已应用筛选条件，适合直接进行目标岗位维护。' : '第一版先承接岗位查、增、改、启停与删除保护链路。' }}</span>
          </div>

          <div class="post-management__table-pills">
            <span class="post-management__table-pill">第 {{ pagination.page }} / {{ pageCount }} 页</span>
            <span class="post-management__table-pill post-management__table-pill--success">启用 {{ enabledCount }}</span>
            <span class="post-management__table-pill post-management__table-pill--danger">禁用 {{ disabledCount }}</span>
          </div>
        </div>
      </template>

      <el-table
        v-loading="loading"
        :data="postList"
        row-key="id"
      >
        <el-table-column
          prop="name"
          label="岗位名称"
          min-width="150"
        />
        <el-table-column
          prop="code"
          label="岗位编码"
          min-width="160"
        />
        <el-table-column
          prop="sort"
          label="排序"
          width="90"
        />
        <el-table-column
          label="绑定用户"
          width="100"
        >
          <template #default="{ row }">
            {{ row.user_count }}
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
          prop="remark"
          label="备注"
          min-width="180"
        >
          <template #default="{ row }">
            {{ row.remark ?? '无' }}
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
          width="260"
        >
          <template #default="{ row }">
            <div class="post-management__row-actions">
              <el-button
                link
                type="primary"
                class="post-management__row-action"
                @click="handleView(row.id)"
              >
                <el-icon><View /></el-icon>
                <span>详情</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="post-management__row-action"
                @click="handleEdit(row.id)"
              >
                <el-icon><EditPen /></el-icon>
                <span>编辑</span>
              </el-button>
              <el-button
                link
                :type="row.status === 1 ? 'danger' : 'success'"
                class="post-management__row-action"
                @click="handleToggleStatus(row)"
              >
                <el-icon><CircleCheck /></el-icon>
                <span>{{ row.status === 1 ? '停用' : '启用' }}</span>
              </el-button>
              <el-button
                link
                type="danger"
                class="post-management__row-action"
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
      title="岗位详情"
      size="520px"
    >
      <div
        v-loading="detailLoading"
        class="post-management__detail"
      >
        <template v-if="currentDetail">
          <div class="post-management__detail-hero-shell post-management__panel">
            <div class="post-management__detail-hero">
              <div class="post-management__detail-avatar">
                {{ currentDetail.name.slice(0, 1) }}
              </div>
              <div class="post-management__detail-copy">
                <strong>{{ currentDetail.name }}</strong>
                <span>{{ currentDetail.code }}</span>
              </div>
              <el-tag :type="currentDetail.status === 1 ? 'success' : 'danger'">
                {{ currentDetail.status === 1 ? '启用' : '禁用' }}
              </el-tag>
            </div>

            <div class="post-management__detail-meta">
              <article>
                <span>岗位排序</span>
                <strong>{{ currentDetail.sort }}</strong>
              </article>
              <article>
                <span>绑定用户</span>
                <strong>{{ currentDetail.user_count }} 人</strong>
              </article>
              <article>
                <span>当前状态</span>
                <strong>{{ currentDetail.status === 1 ? '启用' : '禁用' }}</strong>
              </article>
            </div>
          </div>

          <section class="post-management__detail-section post-management__panel">
            <header class="post-management__detail-section-header">
              <strong>基础资料</strong>
              <span>岗位名称、编码、排序和备注等基本信息。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="岗位名称">
                {{ currentDetail.name }}
              </el-descriptions-item>
              <el-descriptions-item label="岗位编码">
                {{ currentDetail.code }}
              </el-descriptions-item>
              <el-descriptions-item label="排序">
                {{ currentDetail.sort }}
              </el-descriptions-item>
              <el-descriptions-item label="备注">
                {{ currentDetail.remark ?? '无' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="post-management__detail-section post-management__panel">
            <header class="post-management__detail-section-header">
              <strong>关联状态</strong>
              <span>用于确认岗位是否已被用户引用，避免误删基础组织数据。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="绑定用户数">
                {{ currentDetail.user_count }}
              </el-descriptions-item>
              <el-descriptions-item label="当前状态">
                {{ currentDetail.status === 1 ? '启用' : '禁用' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="post-management__detail-section post-management__panel">
            <header class="post-management__detail-section-header">
              <strong>审计信息</strong>
              <span>用于确认岗位创建时间和最近更新时间。</span>
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
      width="720px"
      @cancel="resetForm"
    >
      <div
        v-loading="modalLoading"
        class="post-management__modal-body"
      >
        <el-form
          ref="formRef"
          :model="form"
          :rules="formRules"
          label-position="top"
          class="post-management__form"
        >
          <section class="post-management__form-section post-management__panel">
            <strong>基础信息</strong>
            <span>维护岗位标识、展示名称和排序权重。</span>

            <div class="post-management__form-grid">
              <el-form-item
                v-if="isCreateMode"
                label="岗位编码"
                prop="code"
              >
                <el-input
                  v-model="form.code"
                  placeholder="请输入岗位编码，例如 SYSTEM_ADMIN"
                />
              </el-form-item>

              <el-form-item
                v-else
                label="岗位编码"
                class="post-management__form-span-2"
              >
                <el-input
                  :model-value="currentEditingCode"
                  readonly
                />
              </el-form-item>

              <el-form-item
                label="岗位名称"
                prop="name"
              >
                <el-input
                  v-model="form.name"
                  placeholder="请输入岗位名称"
                />
              </el-form-item>

              <el-form-item label="排序">
                <el-input-number
                  v-model="form.sort"
                  :min="0"
                  :max="999"
                  controls-position="right"
                  class="post-management__number-input"
                />
              </el-form-item>
            </div>
          </section>

          <section class="post-management__form-section post-management__panel">
            <strong>状态与备注</strong>
            <span>岗位停用后不建议再分配给新增用户，历史关联仍保留。</span>

            <div class="post-management__form-grid">
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
                class="post-management__form-span-2"
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
          {{ isCreateMode ? '创建岗位' : '保存修改' }}
        </el-button>
      </template>
    </PageModal>
  </PageContent>
</template>

<style scoped>
.post-management__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.post-management__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  position: relative;
  overflow: hidden;
}

.post-management__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
  background: var(--app-primary);
}

.post-management__overview-card--success::before {
  background: var(--app-success);
}

.post-management__overview-card--warning::before {
  background: var(--app-warning);
}

.post-management__overview-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.post-management__overview-head em {
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.post-management__overview-card--success .post-management__overview-head em {
  background: color-mix(in srgb, var(--app-success) 10%, white);
  color: var(--app-success);
}

.post-management__overview-card--warning .post-management__overview-head em {
  background: color-mix(in srgb, var(--app-warning) 14%, white);
  color: #b45309;
}

.post-management__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.post-management__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.post-management__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.post-management__search-form {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.post-management__search-shell {
  display: grid;
  gap: 14px;
}

.post-management__section-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.post-management__section-copy {
  display: grid;
  gap: 4px;
}

.post-management__section-copy strong {
  color: #111827;
  font-size: 15px;
}

.post-management__section-copy span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.post-management__section-side {
  display: flex;
  align-items: center;
}

.post-management__section-badge {
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

.post-management__section-badge--active {
  border-color: color-mix(in srgb, var(--app-primary) 18%, white);
  background: var(--app-primary-soft);
  color: var(--app-primary);
}

.post-management__search-tips {
  display: flex;
  gap: 10px 18px;
  flex-wrap: wrap;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.post-management__search-actions {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.post-management__search-summary {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.post-management__search-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.post-management__search-form :deep(.el-form-item) {
  margin-bottom: 0;
}

.post-management__table-toolbar {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.post-management__table-meta {
  display: grid;
  gap: 4px;
}

.post-management__table-meta strong {
  color: #111827;
  font-size: 15px;
}

.post-management__table-meta span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.post-management__table-pills {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.post-management__table-pill {
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

.post-management__table-pill--success {
  background: #edf9f1;
  color: #1f8f55;
}

.post-management__table-pill--danger {
  background: #fdf0f0;
  color: #c24141;
}

.post-management__row-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: nowrap;
}

.post-management__row-action {
  margin-left: 0;
  justify-content: flex-start;
  padding: 2px 0;
  flex-shrink: 0;
}

.post-management__panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-md);
  background: var(--app-surface-strong);
}

.post-management__detail {
  display: grid;
  gap: 16px;
  min-height: 240px;
}

.post-management__detail-hero-shell {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.post-management__detail-hero {
  display: flex;
  align-items: center;
  gap: 12px;
}

.post-management__detail-avatar {
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

.post-management__detail-copy {
  display: grid;
  gap: 4px;
  flex: 1;
}

.post-management__detail-copy strong {
  color: #111827;
  font-size: 18px;
}

.post-management__detail-copy span {
  color: var(--app-text-soft);
  font-size: 13px;
}

.post-management__detail-meta {
  display: grid;
  gap: 10px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.post-management__detail-meta article {
  display: grid;
  gap: 6px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.post-management__detail-meta article span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.post-management__detail-meta article strong {
  color: #111827;
  font-size: 14px;
  line-height: 1.6;
}

.post-management__detail-section {
  display: grid;
  gap: 10px;
  padding: 18px;
}

.post-management__detail-section-header {
  display: grid;
  gap: 4px;
}

.post-management__detail-section-header strong {
  color: #111827;
  font-size: 14px;
}

.post-management__detail-section-header span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.post-management__modal-body {
  min-height: 220px;
}

.post-management__form {
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr;
}

.post-management__form-section {
  display: grid;
  gap: 14px;
  padding: 18px;
}

.post-management__form-section strong {
  color: #111827;
  font-size: 14px;
}

.post-management__form-section span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.post-management__form-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.post-management__form-span-2 {
  grid-column: 1 / -1;
}

.post-management__number-input {
  width: 100%;
}

.post-management__form :deep(.el-form-item) {
  margin-bottom: 0;
}

@media (max-width: 1080px) {
  .post-management__overview,
  .post-management__search-form,
  .post-management__form-grid {
    grid-template-columns: 1fr;
  }

  .post-management__search-actions {
    align-items: flex-start;
  }

  .post-management__detail-meta {
    grid-template-columns: 1fr;
  }

  .post-management__form-span-2 {
    grid-column: auto;
  }
}
</style>
