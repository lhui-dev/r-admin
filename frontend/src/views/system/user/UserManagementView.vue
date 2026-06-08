<script setup lang="ts">
import {
  CircleCheck,
  EditPen,
  Plus,
  RefreshRight,
  Search,
  UserFilled,
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
  createSystemUser,
  fetchSystemUserDetail,
  fetchSystemUsers,
  updateSystemUser,
  updateSystemUserRoles,
  updateSystemUserStatus,
  type CreateUserPayload,
  type UpdateUserRolesPayload,
  type UpdateUserPayload,
  type UserDetailData,
  type UserListItem,
} from '@/api/system-user'
import { fetchSystemRoles, type RoleListItem as SystemRoleListItem } from '@/api/system-role'

type UserFormModel = {
  username: string
  password: string
  nickname: string
  realName: string
  mobile: string
  email: string
  gender?: number
  deptId?: number
  roleIds: number[]
  status: number
  remark: string
}

const deptOptions = [
  { label: '平台总部', value: 100 },
  { label: '技术中心', value: 110 },
  { label: '运营中心', value: 120 },
]

const genderOptions = [
  { label: '未知', value: 0 },
  { label: '男', value: 1 },
  { label: '女', value: 2 },
]

const statusOptions = [
  { label: '启用', value: 1 },
  { label: '禁用', value: 0 },
]

const filters = reactive({
  keyword: '',
  deptId: undefined as number | undefined,
  status: undefined as number | undefined,
})

const pagination = reactive({
  page: 1,
  pageSize: 10,
  total: 0,
})

const userList = ref<UserListItem[]>([])
const loading = ref(false)
const roleOptions = ref<SystemRoleListItem[]>([])
const roleOptionsLoading = ref(false)
const detailVisible = ref(false)
const detailLoading = ref(false)
const currentDetail = ref<UserDetailData | null>(null)
const modalVisible = ref(false)
const modalLoading = ref(false)
const submitLoading = ref(false)
const roleAssignVisible = ref(false)
const roleAssignLoading = ref(false)
const roleAssignSubmitLoading = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingUserId = ref<number | null>(null)
const assigningUser = ref<{ id: number, nickname: string } | null>(null)
const formRef = ref<FormInstance>()

const form = reactive<UserFormModel>(createDefaultForm())
const assignRoleIds = ref<number[]>([])

const formRules: FormRules<UserFormModel> = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入初始密码', trigger: 'blur' },
  ],
  nickname: [
    { required: true, message: '请输入用户昵称', trigger: 'blur' },
  ],
}

const isCreateMode = computed(() => modalMode.value === 'create')
const modalTitle = computed(() => (isCreateMode.value ? '新增用户' : '编辑用户'))
const enabledCount = computed(() => userList.value.filter(item => item.status === 1).length)
const disabledCount = computed(() => userList.value.filter(item => item.status === 0).length)
const pageCount = computed(() => Math.max(1, Math.ceil(pagination.total / pagination.pageSize)))
const activeFilterCount = computed(() => [filters.keyword, filters.deptId, filters.status].filter(value => value !== '' && value !== undefined).length)
const hasActiveFilters = computed(() => activeFilterCount.value > 0)
const currentEditingUsername = computed(() => {
  if (!editingUserId.value) {
    return ''
  }

  return userList.value.find(item => item.id === editingUserId.value)?.username ?? ''
})

function createDefaultForm(): UserFormModel {
  return {
    username: '',
    password: '',
    nickname: '',
    realName: '',
    mobile: '',
    email: '',
    gender: 1,
    deptId: 110,
    roleIds: [],
    status: 1,
    remark: '',
  }
}

function resetForm() {
  Object.assign(form, createDefaultForm())
  editingUserId.value = null
  formRef.value?.clearValidate()
}

async function loadRoleOptions() {
  roleOptionsLoading.value = true

  try {
    const response = await fetchSystemRoles({
      page: 1,
      page_size: 100,
    })

    roleOptions.value = response.data.items
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '角色选项加载失败'))
  }
  finally {
    roleOptionsLoading.value = false
  }
}

async function loadUsers() {
  loading.value = true

  try {
    const response = await fetchSystemUsers({
      page: pagination.page,
      page_size: pagination.pageSize,
      keyword: filters.keyword || undefined,
      dept_id: filters.deptId,
      status: filters.status,
    })

    userList.value = response.data.items
    pagination.total = response.data.pagination.total
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '用户列表加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleSearch() {
  pagination.page = 1
  void loadUsers()
}

function handleReset() {
  filters.keyword = ''
  filters.deptId = undefined
  filters.status = undefined
  pagination.page = 1
  void loadUsers()
}

function handlePageChange(page: number) {
  pagination.page = page
  void loadUsers()
}

function handlePageSizeChange(pageSize: number) {
  pagination.page = 1
  pagination.pageSize = pageSize
  void loadUsers()
}

async function handleView(userId: number) {
  detailVisible.value = true
  detailLoading.value = true

  try {
    const response = await fetchSystemUserDetail(userId)
    currentDetail.value = response.data
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '用户详情加载失败'))
  }
  finally {
    detailLoading.value = false
  }
}

function handleCreate() {
  modalMode.value = 'create'
  resetForm()
  void loadRoleOptions()
  modalVisible.value = true
}

function handleCloseModal() {
  modalVisible.value = false
  resetForm()
}

async function handleEdit(userId: number) {
  modalMode.value = 'edit'
  resetForm()
  modalVisible.value = true
  modalLoading.value = true
  void loadRoleOptions()

  try {
    const response = await fetchSystemUserDetail(userId)
    const detail = response.data

    editingUserId.value = detail.id
    form.nickname = detail.nickname
    form.realName = detail.real_name ?? ''
    form.mobile = detail.mobile ?? ''
    form.email = detail.email ?? ''
    form.gender = detail.gender ?? 1
    form.deptId = detail.dept?.id
    form.roleIds = detail.roles.map(role => role.id)
    form.remark = detail.remark ?? ''
  }
  catch (error) {
    modalVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '用户详情加载失败'))
  }
  finally {
    modalLoading.value = false
  }
}

async function handleAssignRoles(userId: number) {
  roleAssignVisible.value = true
  roleAssignLoading.value = true
  assignRoleIds.value = []
  assigningUser.value = null

  try {
    const [detailResponse] = await Promise.all([
      fetchSystemUserDetail(userId),
      loadRoleOptions(),
    ])
    const detail = detailResponse.data

    assigningUser.value = {
      id: detail.id,
      nickname: detail.nickname,
    }
    assignRoleIds.value = detail.roles.map(role => role.id)
  }
  catch (error) {
    roleAssignVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '角色信息加载失败'))
  }
  finally {
    roleAssignLoading.value = false
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
      const payload: CreateUserPayload = {
        username: form.username.trim(),
        password: form.password.trim(),
        nickname: form.nickname.trim(),
        real_name: optionalText(form.realName),
        mobile: optionalText(form.mobile),
        email: optionalText(form.email),
        gender: form.gender,
        dept_id: form.deptId,
        status: form.status,
        role_ids: [...form.roleIds],
        remark: optionalText(form.remark),
      }

      await createSystemUser(payload)
      filters.keyword = payload.username
      pagination.page = 1
      ElMessage.success('用户创建成功')
    }
    else if (editingUserId.value) {
      const payload: UpdateUserPayload = {
        nickname: optionalText(form.nickname),
        real_name: optionalText(form.realName),
        mobile: optionalText(form.mobile),
        email: optionalText(form.email),
        gender: form.gender,
        dept_id: form.deptId,
        role_ids: [...form.roleIds],
        remark: optionalText(form.remark),
      }

      await updateSystemUser(editingUserId.value, payload)
      ElMessage.success('用户更新成功')
    }

    modalVisible.value = false
    await loadUsers()
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, isCreateMode.value ? '用户创建失败' : '用户更新失败'))
  }
  finally {
    submitLoading.value = false
  }
}

async function handleSubmitAssignedRoles() {
  if (!assigningUser.value) {
    return
  }

  const targetUserId = assigningUser.value.id
  roleAssignSubmitLoading.value = true

  try {
    const payload: UpdateUserRolesPayload = {
      role_ids: [...assignRoleIds.value],
    }

    await updateSystemUserRoles(targetUserId, payload)
    ElMessage.success('角色分配已更新')
    roleAssignVisible.value = false

    await loadUsers()
    if (currentDetail.value?.id === targetUserId) {
      const response = await fetchSystemUserDetail(targetUserId)
      currentDetail.value = response.data
    }
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '角色分配失败'))
  }
  finally {
    roleAssignSubmitLoading.value = false
  }
}

async function handleToggleStatus(row: UserListItem) {
  const nextStatus = row.status === 1 ? 0 : 1
  const actionText = nextStatus === 1 ? '启用' : '停用'

  try {
    await ElMessageBox.confirm(
      `确认${actionText}用户 “${row.nickname}” 吗？`,
      `${actionText}确认`,
      {
        type: 'warning',
        confirmButtonText: `确认${actionText}`,
        cancelButtonText: '取消',
      },
    )

    await updateSystemUserStatus(row.id, nextStatus)
    ElMessage.success(`用户已${actionText}`)
    await loadUsers()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, `${actionText}失败`))
  }
}

function handleCloseAssignRoles() {
  roleAssignVisible.value = false
  assigningUser.value = null
  assignRoleIds.value = []
}

function optionalText(value: string) {
  const trimmed = value.trim()
  return trimmed ? trimmed : undefined
}

function formatDateTime(value?: string | null) {
  if (!value) {
    return '未记录'
  }

  return value.replace('T', ' ').replace('Z', ' UTC')
}

function formatRoles(row: UserListItem) {
  if (!row.roles.length) {
    return '未分配'
  }

  return row.roles.map(role => role.name).join(' / ')
}

function formatPosts(row: UserListItem) {
  if (!row.posts.length) {
    return '未分配'
  }

  return row.posts.map(post => post.name).join(' / ')
}

function formatRoleNames(roles: Array<{ name: string }>) {
  if (!roles.length) {
    return '未分配'
  }

  return roles.map(role => role.name).join(' / ')
}

function formatRoleOptionLabel(role: SystemRoleListItem) {
  return `${role.name} (${role.code})`
}

function formatPostNames(posts: Array<{ name: string }>) {
  if (!posts.length) {
    return '未分配'
  }

  return posts.map(post => post.name).join(' / ')
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
  void loadUsers()
  void loadRoleOptions()
})
</script>

<template>
  <PageContent
    eyebrow="System User"
    title="用户管理"
    description="基于后端第一版用户管理接口，先完成列表、详情、创建、编辑与启停的前端闭环，并把查询区、表格区沉淀到 base-ui。"
  >
    <template #actions>
      <el-button
        type="primary"
        @click="handleCreate"
      >
        <el-icon><Plus /></el-icon>
        <span>新增用户</span>
      </el-button>
      <el-button @click="loadUsers">
        <el-icon><RefreshRight /></el-icon>
        <span>刷新列表</span>
      </el-button>
    </template>

    <section class="user-management__overview">
      <article class="user-management__overview-card user-management__overview-card--total">
        <div class="user-management__overview-head">
          <span>当前分页总量</span>
          <em>总览</em>
        </div>
        <strong>{{ pagination.total }}</strong>
        <p>与当前筛选条件匹配的用户总数</p>
      </article>

      <article class="user-management__overview-card user-management__overview-card--success">
        <div class="user-management__overview-head">
          <span>当前页启用用户</span>
          <em>正常</em>
        </div>
        <strong>{{ enabledCount }}</strong>
        <p>用于快速判断当前页活跃用户状态</p>
      </article>

      <article class="user-management__overview-card user-management__overview-card--danger">
        <div class="user-management__overview-head">
          <span>当前页禁用用户</span>
          <em>关注</em>
        </div>
        <strong>{{ disabledCount }}</strong>
        <p>可结合筛选快速进行启停维护</p>
      </article>
    </section>

    <PageSearch>
      <div class="user-management__search-shell">
        <div class="user-management__section-heading">
          <div class="user-management__section-copy">
            <strong>条件检索</strong>
            <span>先收窄范围，再进入创建、编辑和状态维护。</span>
          </div>

          <div class="user-management__section-side">
            <span
              class="user-management__section-badge"
              :class="{ 'user-management__section-badge--active': hasActiveFilters }"
            >
              {{ hasActiveFilters ? `已启用 ${activeFilterCount} 个筛选条件` : '当前为全部数据' }}
            </span>
          </div>
        </div>

        <el-form
          class="user-management__search-form"
          :model="filters"
          label-position="top"
        >
          <el-form-item label="关键词">
            <el-input
              v-model="filters.keyword"
              placeholder="用户名 / 昵称 / 手机号"
              clearable
              @keyup.enter="handleSearch"
            />
          </el-form-item>

          <el-form-item label="所属部门">
            <el-select
              v-model="filters.deptId"
              placeholder="全部部门"
              clearable
            >
              <el-option
                v-for="option in deptOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
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

        <div class="user-management__search-tips">
          <span>支持按用户名、昵称、手机号快速定位账号。</span>
          <span>适合先按部门或状态收窄，再做精确检索。</span>
        </div>
      </div>

      <template #actions>
        <div class="user-management__search-actions">
          <p class="user-management__search-summary">
            当前每页 {{ pagination.pageSize }} 条，共 {{ pagination.total }} 条结果
          </p>

          <div class="user-management__search-buttons">
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
        <div class="user-management__table-toolbar">
          <div class="user-management__table-meta">
            <strong>用户列表</strong>
            <span>{{ hasActiveFilters ? '当前结果已应用筛选条件，适合直接进行目标维护。' : '第一版先承接最常用的查、增、改、启停链路。' }}</span>
          </div>

          <div class="user-management__table-pills">
            <span class="user-management__table-pill">第 {{ pagination.page }} / {{ pageCount }} 页</span>
            <span class="user-management__table-pill user-management__table-pill--success">启用 {{ enabledCount }}</span>
            <span class="user-management__table-pill user-management__table-pill--danger">禁用 {{ disabledCount }}</span>
          </div>
        </div>
      </template>

      <el-table
        v-loading="loading"
        :data="userList"
        row-key="id"
      >
        <el-table-column
          prop="username"
          label="用户名"
          min-width="140"
        />
        <el-table-column
          prop="nickname"
          label="昵称"
          min-width="140"
        />
        <el-table-column
          prop="dept.name"
          label="部门"
          min-width="120"
        >
          <template #default="{ row }">
            {{ row.dept?.name ?? '未归属' }}
          </template>
        </el-table-column>
        <el-table-column
          label="角色"
          min-width="160"
        >
          <template #default="{ row }">
            {{ formatRoles(row) }}
          </template>
        </el-table-column>
        <el-table-column
          label="岗位"
          min-width="140"
        >
          <template #default="{ row }">
            {{ formatPosts(row) }}
          </template>
        </el-table-column>
        <el-table-column
          prop="mobile"
          label="手机号"
          min-width="130"
        />
        <el-table-column
          prop="email"
          label="邮箱"
          min-width="180"
        />
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
            <el-button
              link
              type="primary"
              @click="handleView(row.id)"
            >
              <el-icon><View /></el-icon>
              <span>详情</span>
            </el-button>
            <el-button
              link
              type="primary"
              @click="handleEdit(row.id)"
            >
              <el-icon><EditPen /></el-icon>
              <span>编辑</span>
            </el-button>
            <el-button
              link
              type="primary"
              @click="handleAssignRoles(row.id)"
            >
              <el-icon><UserFilled /></el-icon>
              <span>分配角色</span>
            </el-button>
            <el-button
              link
              :type="row.status === 1 ? 'danger' : 'success'"
              @click="handleToggleStatus(row)"
            >
              <el-icon><CircleCheck /></el-icon>
              <span>{{ row.status === 1 ? '停用' : '启用' }}</span>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </BaseTable>

    <el-drawer
      v-model="detailVisible"
      title="用户详情"
      size="520px"
    >
      <div
        v-loading="detailLoading"
        class="user-management__detail"
      >
        <template v-if="currentDetail">
          <div class="user-management__detail-hero-shell user-management__panel">
            <div class="user-management__detail-hero">
              <div class="user-management__detail-avatar">
                {{ currentDetail.nickname.slice(0, 1) }}
              </div>
              <div class="user-management__detail-copy">
                <strong>{{ currentDetail.nickname }}</strong>
                <span>{{ currentDetail.username }}</span>
              </div>
              <el-tag :type="currentDetail.status === 1 ? 'success' : 'danger'">
                {{ currentDetail.status === 1 ? '启用' : '禁用' }}
              </el-tag>
            </div>

            <div class="user-management__detail-meta">
              <article>
                <span>部门</span>
                <strong>{{ currentDetail.dept?.name ?? '未归属' }}</strong>
              </article>
              <article>
                <span>角色</span>
                <strong>{{ currentDetail.roles.length }} 个</strong>
              </article>
              <article>
                <span>岗位</span>
                <strong>{{ currentDetail.posts.length }} 个</strong>
              </article>
            </div>
          </div>

          <section class="user-management__detail-section user-management__panel">
            <header class="user-management__detail-section-header">
              <strong>基础资料</strong>
              <span>账号基础身份信息与联系方式。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="真实姓名">
                {{ currentDetail.real_name ?? '未填写' }}
              </el-descriptions-item>
              <el-descriptions-item label="手机号">
                {{ currentDetail.mobile ?? '未填写' }}
              </el-descriptions-item>
              <el-descriptions-item label="邮箱">
                {{ currentDetail.email ?? '未填写' }}
              </el-descriptions-item>
              <el-descriptions-item label="备注">
                {{ currentDetail.remark ?? '无' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="user-management__detail-section user-management__panel">
            <header class="user-management__detail-section-header">
              <strong>组织与授权</strong>
              <span>用于确认当前用户的组织归属和权限挂载情况。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="部门">
                {{ currentDetail.dept?.name ?? '未归属' }}
              </el-descriptions-item>
              <el-descriptions-item label="角色">
                {{ formatRoleNames(currentDetail.roles) }}
              </el-descriptions-item>
              <el-descriptions-item label="岗位">
                {{ formatPostNames(currentDetail.posts) }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="user-management__detail-section user-management__panel">
            <header class="user-management__detail-section-header">
              <strong>审计与安全</strong>
              <span>保留账号状态、最后登录与密码更新时间等信息。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="最后登录时间">
                {{ formatDateTime(currentDetail.last_login_at) }}
              </el-descriptions-item>
              <el-descriptions-item label="密码更新时间">
                {{ formatDateTime(currentDetail.password_updated_at) }}
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
        class="user-management__modal-body"
      >
        <el-form
          ref="formRef"
          :model="form"
          :rules="formRules"
          label-position="top"
          class="user-management__form"
        >
          <section class="user-management__form-section user-management__panel">
            <strong>基础信息</strong>
            <span>维护账号身份、展示名称和主要联系方式。</span>

            <div class="user-management__form-grid">
              <el-form-item
                v-if="isCreateMode"
                label="用户名"
                prop="username"
              >
                <el-input
                  v-model="form.username"
                  placeholder="请输入用户名"
                />
              </el-form-item>

              <el-form-item
                v-if="isCreateMode"
                label="初始密码"
                prop="password"
              >
                <el-input
                  v-model="form.password"
                  type="password"
                  show-password
                  placeholder="请输入初始密码，建议使用 8 位以上强密码"
                />
              </el-form-item>

              <el-form-item
                v-else
                label="用户名"
                class="user-management__form-span-2"
              >
                <el-input
                  :model-value="currentEditingUsername"
                  readonly
                />
              </el-form-item>

              <el-form-item
                label="用户昵称"
                prop="nickname"
              >
                <el-input
                  v-model="form.nickname"
                  placeholder="请输入用户昵称"
                />
              </el-form-item>

              <el-form-item label="真实姓名">
                <el-input
                  v-model="form.realName"
                  placeholder="请输入真实姓名"
                />
              </el-form-item>

              <el-form-item label="手机号">
                <el-input
                  v-model="form.mobile"
                  placeholder="请输入手机号"
                />
              </el-form-item>

              <el-form-item label="邮箱">
                <el-input
                  v-model="form.email"
                  placeholder="请输入邮箱"
                />
              </el-form-item>

              <el-form-item label="性别">
                <el-select v-model="form.gender">
                  <el-option
                    v-for="option in genderOptions"
                    :key="option.value"
                    :label="option.label"
                    :value="option.value"
                  />
                </el-select>
              </el-form-item>
            </div>
          </section>

          <section class="user-management__form-section user-management__panel">
            <strong>组织与状态</strong>
            <span>设置部门归属、角色绑定，并确认新建账号的启用状态。</span>

            <div class="user-management__form-grid">
              <el-form-item label="部门">
                <el-select v-model="form.deptId">
                  <el-option
                    v-for="option in deptOptions"
                    :key="option.value"
                    :label="option.label"
                    :value="option.value"
                  />
                </el-select>
              </el-form-item>

              <el-form-item
                label="角色"
                class="user-management__form-span-2"
              >
                <el-select
                  v-model="form.roleIds"
                  multiple
                  collapse-tags
                  collapse-tags-tooltip
                  placeholder="请选择角色"
                  :loading="roleOptionsLoading"
                >
                  <el-option
                    v-for="role in roleOptions"
                    :key="role.id"
                    :label="formatRoleOptionLabel(role)"
                    :value="role.id"
                    :disabled="role.status !== 1"
                  />
                </el-select>
              </el-form-item>

              <el-form-item
                v-if="isCreateMode"
                label="状态"
              >
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
                class="user-management__form-span-2"
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
          {{ isCreateMode ? '创建用户' : '保存修改' }}
        </el-button>
      </template>
    </PageModal>

    <PageModal
      v-model="roleAssignVisible"
      title="分配角色"
      width="640px"
      @cancel="handleCloseAssignRoles"
    >
      <div
        v-loading="roleAssignLoading"
        class="user-management__modal-body"
      >
        <section class="user-management__form-section user-management__panel">
          <strong>{{ assigningUser?.nickname ?? '目标用户' }}</strong>
          <span>独立维护当前用户的角色集合，保存后会直接覆盖原有角色关系。</span>

          <el-form label-position="top">
            <el-form-item label="角色选择">
              <el-select
                v-model="assignRoleIds"
                multiple
                collapse-tags
                collapse-tags-tooltip
                placeholder="请选择角色"
                :loading="roleOptionsLoading"
              >
                <el-option
                  v-for="role in roleOptions"
                  :key="role.id"
                  :label="formatRoleOptionLabel(role)"
                  :value="role.id"
                  :disabled="role.status !== 1"
                />
              </el-select>
            </el-form-item>
          </el-form>
        </section>
      </div>

      <template #footer>
        <el-button @click="handleCloseAssignRoles">
          取消
        </el-button>
        <el-button
          type="primary"
          :loading="roleAssignSubmitLoading"
          @click="handleSubmitAssignedRoles"
        >
          保存角色
        </el-button>
      </template>
    </PageModal>
  </PageContent>
</template>

<style scoped>
.user-management__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.user-management__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  position: relative;
  overflow: hidden;
}

.user-management__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
  background: var(--app-primary);
}

.user-management__overview-card--success::before {
  background: var(--app-success);
}

.user-management__overview-card--danger::before {
  background: var(--app-danger);
}

.user-management__overview-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.user-management__overview-head em {
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.user-management__overview-card--success .user-management__overview-head em {
  background: color-mix(in srgb, var(--app-success) 10%, white);
  color: var(--app-success);
}

.user-management__overview-card--danger .user-management__overview-head em {
  background: color-mix(in srgb, var(--app-danger) 10%, white);
  color: var(--app-danger);
}

.user-management__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.user-management__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.user-management__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.user-management__search-form {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.user-management__search-shell {
  display: grid;
  gap: 14px;
}

.user-management__section-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.user-management__section-copy {
  display: grid;
  gap: 4px;
}

.user-management__section-copy strong {
  color: #111827;
  font-size: 15px;
}

.user-management__section-copy span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.user-management__section-side {
  display: flex;
  align-items: center;
}

.user-management__section-badge {
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

.user-management__section-badge--active {
  border-color: color-mix(in srgb, var(--app-primary) 18%, white);
  background: var(--app-primary-soft);
  color: var(--app-primary);
}

.user-management__search-tips {
  display: flex;
  gap: 10px 18px;
  flex-wrap: wrap;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.user-management__search-actions {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.user-management__search-summary {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.user-management__search-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.user-management__search-form :deep(.el-form-item) {
  margin-bottom: 0;
}

.user-management__table-toolbar {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.user-management__table-meta {
  display: grid;
  gap: 4px;
}

.user-management__table-meta strong {
  color: #111827;
  font-size: 15px;
}

.user-management__table-meta span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.user-management__table-pills {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.user-management__table-pill {
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

.user-management__table-pill--success {
  background: #edf9f1;
  color: #1f8f55;
}

.user-management__table-pill--danger {
  background: #fdf0f0;
  color: #c24141;
}

.user-management__panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-md);
  background: var(--app-surface-strong);
}

.user-management__detail {
  display: grid;
  gap: 16px;
  min-height: 240px;
}

.user-management__detail-hero-shell {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.user-management__detail-hero {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-management__detail-avatar {
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

.user-management__detail-copy {
  display: grid;
  gap: 4px;
  flex: 1;
}

.user-management__detail-copy strong {
  color: #111827;
  font-size: 18px;
}

.user-management__detail-copy span {
  color: var(--app-text-soft);
  font-size: 13px;
}

.user-management__detail-meta {
  display: grid;
  gap: 10px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.user-management__detail-meta article {
  display: grid;
  gap: 6px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.user-management__detail-meta article span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.user-management__detail-meta article strong {
  color: #111827;
  font-size: 14px;
  line-height: 1.6;
}

.user-management__detail-section {
  display: grid;
  gap: 10px;
  padding: 18px;
}

.user-management__detail-section-header {
  display: grid;
  gap: 4px;
}

.user-management__detail-section-header strong {
  color: #111827;
  font-size: 14px;
}

.user-management__detail-section-header span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.user-management__modal-body {
  min-height: 220px;
}

.user-management__form {
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr;
}

.user-management__form-section {
  display: grid;
  gap: 14px;
  padding: 18px;
}

.user-management__form-section strong {
  color: #111827;
  font-size: 14px;
}

.user-management__form-section span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.user-management__form-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.user-management__form-span-2 {
  grid-column: 1 / -1;
}

.user-management__form :deep(.el-form-item) {
  margin-bottom: 0;
}

@media (max-width: 1080px) {
  .user-management__overview,
  .user-management__search-form,
  .user-management__form-grid {
    grid-template-columns: 1fr;
  }

  .user-management__search-actions {
    align-items: flex-start;
  }

  .user-management__detail-meta {
    grid-template-columns: 1fr;
  }

  .user-management__form-span-2 {
    grid-column: auto;
  }
}
</style>
