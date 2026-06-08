<script setup lang="ts">
import {
  CircleCheck,
  Delete,
  EditPen,
  FolderAdd,
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
  createSystemMenu,
  deleteSystemMenu,
  fetchSystemMenuDetail,
  fetchSystemMenuTree,
  updateSystemMenu,
  updateSystemMenuStatus,
  type CreateMenuPayload,
  type MenuTreeItem,
  type MenuType,
  type UpdateMenuPayload,
} from '@/api/system-menu'

type MenuFormModel = {
  parentId: number
  menuName: string
  menuType: MenuType
  routeName: string
  routePath: string
  componentPath: string
  permissionCode: string
  icon: string
  sortNo: number
  visible: boolean
  keepAlive: boolean
  isExternal: boolean
  status: number
  remark: string
}

type ParentOption = {
  id: number
  label: string
  disabled: boolean
}

const menuTypeOptions: Array<{ label: string, value: MenuType, description: string }> = [
  { label: '目录', value: 'catalog', description: '承载一级或分组导航' },
  { label: '菜单', value: 'menu', description: '可访问页面路由' },
  { label: '按钮', value: 'button', description: '页面操作权限点' },
  { label: '接口', value: 'api', description: '后端接口权限点' },
]

const statusOptions = [
  { label: '启用', value: 1 },
  { label: '禁用', value: 0 },
]

const filters = reactive({
  keyword: '',
  status: undefined as number | undefined,
  menuType: undefined as MenuType | undefined,
})

const menuTree = ref<MenuTreeItem[]>([])
const loading = ref(false)
const detailVisible = ref(false)
const detailLoading = ref(false)
const currentDetail = ref<MenuTreeItem | null>(null)
const modalVisible = ref(false)
const modalLoading = ref(false)
const submitLoading = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingMenuId = ref<number | null>(null)
const formRef = ref<FormInstance>()

const form = reactive<MenuFormModel>(createDefaultForm())

const formRules: FormRules<MenuFormModel> = {
  menuName: [
    { required: true, message: '请输入菜单名称', trigger: 'blur' },
  ],
  menuType: [
    { required: true, message: '请选择菜单类型', trigger: 'change' },
  ],
  routePath: [
    {
      validator: (_rule, value, callback) => {
        if (form.menuType === 'menu' && !String(value ?? '').trim()) {
          callback(new Error('菜单类型需要填写路由路径'))
          return
        }

        if (String(value ?? '').trim() && !String(value).trim().startsWith('/')) {
          callback(new Error('路由路径必须以 / 开头'))
          return
        }

        callback()
      },
      trigger: 'blur',
    },
  ],
  permissionCode: [
    {
      validator: (_rule, value, callback) => {
        if ((form.menuType === 'button' || form.menuType === 'api') && !String(value ?? '').trim()) {
          callback(new Error('按钮或接口类型需要填写权限标识'))
          return
        }

        callback()
      },
      trigger: 'blur',
    },
  ],
}

const isCreateMode = computed(() => modalMode.value === 'create')
const modalTitle = computed(() => (isCreateMode.value ? '新增菜单' : '编辑菜单'))
const flatMenus = computed(() => flattenMenuTree(menuTree.value))
const enabledCount = computed(() => flatMenus.value.filter(item => item.status === 1).length)
const disabledCount = computed(() => flatMenus.value.filter(item => item.status === 0).length)
const visibleCount = computed(() => flatMenus.value.filter(item => item.visible).length)
const activeFilterCount = computed(() => [filters.keyword, filters.status, filters.menuType].filter(value => value !== '' && value !== undefined).length)
const hasActiveFilters = computed(() => activeFilterCount.value > 0)
const parentOptions = computed<ParentOption[]>(() => {
  const blockedIds = editingMenuId.value ? collectDescendantIds(editingMenuId.value, menuTree.value) : new Set<number>()
  if (editingMenuId.value) {
    blockedIds.add(editingMenuId.value)
  }

  return [
    { id: 0, label: '根目录', disabled: false },
    ...flatMenus.value
      .filter(item => item.menu_type !== 'button' && item.menu_type !== 'api')
      .map(item => ({
        id: item.id,
        label: `${'　'.repeat(item.level)}${item.menu_name}`,
        disabled: blockedIds.has(item.id),
      })),
  ]
})

function createDefaultForm(): MenuFormModel {
  return {
    parentId: 0,
    menuName: '',
    menuType: 'menu',
    routeName: '',
    routePath: '',
    componentPath: '',
    permissionCode: '',
    icon: 'Menu',
    sortNo: 100,
    visible: true,
    keepAlive: false,
    isExternal: false,
    status: 1,
    remark: '',
  }
}

function resetForm() {
  Object.assign(form, createDefaultForm())
  editingMenuId.value = null
  formRef.value?.clearValidate()
}

async function loadMenus() {
  loading.value = true

  try {
    const response = await fetchSystemMenuTree({
      keyword: filters.keyword || undefined,
      status: filters.status,
      menu_type: filters.menuType,
    })

    menuTree.value = response.data.items
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '菜单树加载失败'))
  }
  finally {
    loading.value = false
  }
}

function handleSearch() {
  void loadMenus()
}

function handleReset() {
  filters.keyword = ''
  filters.status = undefined
  filters.menuType = undefined
  void loadMenus()
}

async function handleView(menuId: number) {
  detailVisible.value = true
  detailLoading.value = true

  try {
    const response = await fetchSystemMenuDetail(menuId)
    currentDetail.value = response.data
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, '菜单详情加载失败'))
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

function handleCreateChild(row: MenuTreeItem) {
  if (row.menu_type === 'button' || row.menu_type === 'api') {
    ElMessage.warning('按钮和接口权限点下不能继续新增子菜单')
    return
  }

  handleCreate(row.id)
}

function handleCloseModal() {
  modalVisible.value = false
  resetForm()
}

async function handleEdit(menuId: number) {
  modalMode.value = 'edit'
  resetForm()
  modalVisible.value = true
  modalLoading.value = true

  try {
    const response = await fetchSystemMenuDetail(menuId)
    const detail = response.data

    editingMenuId.value = detail.id
    form.parentId = detail.parent_id
    form.menuName = detail.menu_name
    form.menuType = detail.menu_type
    form.routeName = detail.route_name ?? ''
    form.routePath = detail.route_path ?? ''
    form.componentPath = detail.component_path ?? ''
    form.permissionCode = detail.permission_code ?? ''
    form.icon = detail.icon ?? ''
    form.sortNo = detail.sort_no
    form.visible = detail.visible
    form.keepAlive = detail.keep_alive
    form.isExternal = detail.is_external
    form.status = detail.status
    form.remark = detail.remark ?? ''
  }
  catch (error) {
    modalVisible.value = false
    ElMessage.error(resolveErrorMessage(error, '菜单详情加载失败'))
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

  const targetMenuId = editingMenuId.value
  submitLoading.value = true

  try {
    if (isCreateMode.value) {
      const payload: CreateMenuPayload = buildCreatePayload()

      await createSystemMenu(payload)
      ElMessage.success('菜单创建成功')
    }
    else if (targetMenuId) {
      const payload: UpdateMenuPayload = buildUpdatePayload()

      await updateSystemMenu(targetMenuId, payload)
      ElMessage.success('菜单更新成功')
    }

    modalVisible.value = false
    await loadMenus()
  }
  catch (error) {
    ElMessage.error(resolveErrorMessage(error, isCreateMode.value ? '菜单创建失败' : '菜单更新失败'))
  }
  finally {
    submitLoading.value = false
  }
}

async function handleToggleStatus(row: MenuTreeItem) {
  const nextStatus = row.status === 1 ? 0 : 1
  const actionText = nextStatus === 1 ? '启用' : '停用'

  try {
    await ElMessageBox.confirm(
      `确认${actionText}菜单 “${row.menu_name}” 吗？`,
      `${actionText}确认`,
      {
        type: 'warning',
        confirmButtonText: `确认${actionText}`,
        cancelButtonText: '取消',
      },
    )

    await updateSystemMenuStatus(row.id, nextStatus)
    ElMessage.success(`菜单已${actionText}`)
    await loadMenus()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, `${actionText}失败`))
  }
}

async function handleDelete(row: MenuTreeItem) {
  try {
    await ElMessageBox.confirm(
      `确认删除菜单 “${row.menu_name}” 吗？如存在子节点或角色授权引用，后端会拒绝删除。`,
      '删除确认',
      {
        type: 'warning',
        confirmButtonText: '确认删除',
        cancelButtonText: '取消',
      },
    )

    await deleteSystemMenu(row.id)
    ElMessage.success('菜单已删除')
    await loadMenus()
  }
  catch (error) {
    if (error === 'cancel' || error === 'close') {
      return
    }

    ElMessage.error(resolveErrorMessage(error, '菜单删除失败'))
  }
}

function buildCreatePayload(): CreateMenuPayload {
  return {
    parent_id: form.parentId,
    menu_name: form.menuName.trim(),
    menu_type: form.menuType,
    route_name: optionalText(form.routeName),
    route_path: optionalText(form.routePath),
    component_path: optionalText(form.componentPath),
    permission_code: optionalText(form.permissionCode),
    icon: optionalText(form.icon),
    sort_no: form.sortNo,
    visible: form.visible,
    keep_alive: form.keepAlive,
    is_external: form.isExternal,
    status: form.status,
    remark: optionalText(form.remark),
  }
}

function buildUpdatePayload(): UpdateMenuPayload {
  return {
    parent_id: form.parentId,
    menu_name: form.menuName.trim(),
    menu_type: form.menuType,
    route_name: form.routeName.trim(),
    route_path: form.routePath.trim(),
    component_path: form.componentPath.trim(),
    permission_code: form.permissionCode.trim(),
    icon: form.icon.trim(),
    sort_no: form.sortNo,
    visible: form.visible,
    keep_alive: form.keepAlive,
    is_external: form.isExternal,
    status: form.status,
    remark: form.remark.trim(),
  }
}

function optionalText(value: string) {
  const trimmed = value.trim()
  return trimmed ? trimmed : undefined
}

function flattenMenuTree(items: MenuTreeItem[], level = 0): Array<MenuTreeItem & { level: number }> {
  return items.flatMap(item => [
    { ...item, level },
    ...flattenMenuTree(item.children, level + 1),
  ])
}

function collectDescendantIds(menuId: number, items: MenuTreeItem[]) {
  const result = new Set<number>()

  function walk(nodes: MenuTreeItem[], active: boolean) {
    for (const item of nodes) {
      const nextActive = active || item.id === menuId
      if (active) {
        result.add(item.id)
      }
      walk(item.children, nextActive)
    }
  }

  walk(items, false)
  return result
}

function resolveMenuTypeLabel(value: MenuType) {
  return menuTypeOptions.find(option => option.value === value)?.label ?? value
}

function resolveMenuTypeTag(value: MenuType) {
  const map: Record<MenuType, 'primary' | 'success' | 'warning' | 'info'> = {
    catalog: 'warning',
    menu: 'primary',
    button: 'success',
    api: 'info',
  }

  return map[value]
}

function formatBoolean(value: boolean) {
  return value ? '是' : '否'
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
  void loadMenus()
})
</script>

<template>
  <PageContent
    eyebrow="System Menu"
    title="菜单管理"
    description="维护后台菜单树、路由路径、权限标识和展示状态，让动态菜单与角色授权拥有统一的数据源。"
  >
    <template #actions>
      <el-button
        type="primary"
        @click="handleCreate()"
      >
        <el-icon><Plus /></el-icon>
        <span>新增菜单</span>
      </el-button>
      <el-button @click="loadMenus">
        <el-icon><RefreshRight /></el-icon>
        <span>刷新菜单</span>
      </el-button>
    </template>

    <section class="menu-management__overview">
      <article class="menu-management__overview-card menu-management__overview-card--total">
        <div class="menu-management__overview-head">
          <span>菜单节点</span>
          <em>总览</em>
        </div>
        <strong>{{ flatMenus.length }}</strong>
        <p>当前筛选条件下可维护的菜单、目录与权限点数量。</p>
      </article>

      <article class="menu-management__overview-card menu-management__overview-card--success">
        <div class="menu-management__overview-head">
          <span>启用节点</span>
          <em>可见链路</em>
        </div>
        <strong>{{ enabledCount }}</strong>
        <p>启用节点会参与菜单树和后续授权配置。</p>
      </article>

      <article class="menu-management__overview-card menu-management__overview-card--warning">
        <div class="menu-management__overview-head">
          <span>显示节点</span>
          <em>导航</em>
        </div>
        <strong>{{ visibleCount }}</strong>
        <p>显示状态决定节点是否进入用户可见导航。</p>
      </article>
    </section>

    <PageSearch>
      <div class="menu-management__search-shell">
        <div class="menu-management__section-heading">
          <div class="menu-management__section-copy">
            <strong>条件检索</strong>
            <span>按名称、路由、权限标识或节点类型定位菜单。</span>
          </div>

          <div class="menu-management__section-side">
            <span
              class="menu-management__section-badge"
              :class="{ 'menu-management__section-badge--active': hasActiveFilters }"
            >
              {{ hasActiveFilters ? `已启用 ${activeFilterCount} 个筛选条件` : '当前为全部菜单' }}
            </span>
          </div>
        </div>

        <el-form
          class="menu-management__search-form"
          :model="filters"
          label-position="top"
        >
          <el-form-item label="关键词">
            <el-input
              v-model="filters.keyword"
              placeholder="菜单名称 / 路由 / 权限标识"
              clearable
              @keyup.enter="handleSearch"
            />
          </el-form-item>

          <el-form-item label="类型">
            <el-select
              v-model="filters.menuType"
              placeholder="全部类型"
              clearable
            >
              <el-option
                v-for="option in menuTypeOptions"
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
      </div>

      <template #actions>
        <div class="menu-management__search-actions">
          <p class="menu-management__search-summary">
            当前共 {{ flatMenus.length }} 个节点，其中 {{ disabledCount }} 个禁用。
          </p>

          <div class="menu-management__search-buttons">
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
      :total="flatMenus.length"
      :page="1"
      :page-size="flatMenus.length || 10"
      :show-pagination="false"
    >
      <template #toolbar>
        <div class="menu-management__table-toolbar">
          <div class="menu-management__table-meta">
            <strong>菜单树</strong>
            <span>{{ hasActiveFilters ? '当前树已应用筛选条件，父级缺失的节点会提升为当前视图根节点。' : '维护目录、菜单、按钮和接口权限点的统一配置。' }}</span>
          </div>

          <div class="menu-management__table-pills">
            <span class="menu-management__table-pill">节点 {{ flatMenus.length }}</span>
            <span class="menu-management__table-pill menu-management__table-pill--success">启用 {{ enabledCount }}</span>
            <span class="menu-management__table-pill menu-management__table-pill--danger">禁用 {{ disabledCount }}</span>
          </div>
        </div>
      </template>

      <el-table
        v-loading="loading"
        :data="menuTree"
        row-key="id"
        default-expand-all
        :tree-props="{ children: 'children' }"
      >
        <el-table-column
          prop="menu_name"
          label="菜单名称"
          min-width="190"
        />
        <el-table-column
          label="类型"
          width="96"
        >
          <template #default="{ row }">
            <el-tag :type="resolveMenuTypeTag(row.menu_type)">
              {{ resolveMenuTypeLabel(row.menu_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column
          prop="route_path"
          label="路由路径"
          min-width="170"
        >
          <template #default="{ row }">
            {{ row.route_path ?? '未配置' }}
          </template>
        </el-table-column>
        <el-table-column
          prop="permission_code"
          label="权限标识"
          min-width="190"
        >
          <template #default="{ row }">
            {{ row.permission_code ?? '未绑定' }}
          </template>
        </el-table-column>
        <el-table-column
          prop="component_path"
          label="组件路径"
          min-width="180"
        >
          <template #default="{ row }">
            {{ row.component_path ?? '未配置' }}
          </template>
        </el-table-column>
        <el-table-column
          prop="sort_no"
          label="排序"
          width="86"
        />
        <el-table-column
          label="显示"
          width="86"
        >
          <template #default="{ row }">
            {{ formatBoolean(row.visible) }}
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
          label="操作"
          fixed="right"
          width="360"
        >
          <template #default="{ row }">
            <div class="menu-management__row-actions">
              <el-button
                link
                type="primary"
                class="menu-management__row-action"
                @click="handleView(row.id)"
              >
                <el-icon><View /></el-icon>
                <span>详情</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="menu-management__row-action"
                @click="handleCreateChild(row)"
              >
                <el-icon><FolderAdd /></el-icon>
                <span>新增子项</span>
              </el-button>
              <el-button
                link
                type="primary"
                class="menu-management__row-action"
                @click="handleEdit(row.id)"
              >
                <el-icon><EditPen /></el-icon>
                <span>编辑</span>
              </el-button>
              <el-button
                link
                :type="row.status === 1 ? 'danger' : 'success'"
                class="menu-management__row-action"
                @click="handleToggleStatus(row)"
              >
                <el-icon><CircleCheck /></el-icon>
                <span>{{ row.status === 1 ? '停用' : '启用' }}</span>
              </el-button>
              <el-button
                link
                type="danger"
                class="menu-management__row-action"
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
      title="菜单详情"
      size="560px"
    >
      <div
        v-loading="detailLoading"
        class="menu-management__detail"
      >
        <template v-if="currentDetail">
          <div class="menu-management__detail-hero-shell menu-management__panel">
            <div class="menu-management__detail-hero">
              <div class="menu-management__detail-avatar">
                {{ currentDetail.menu_name.slice(0, 1) }}
              </div>
              <div class="menu-management__detail-copy">
                <strong>{{ currentDetail.menu_name }}</strong>
                <span>{{ currentDetail.permission_code ?? currentDetail.route_path ?? '未绑定权限或路由' }}</span>
              </div>
              <el-tag :type="currentDetail.status === 1 ? 'success' : 'danger'">
                {{ currentDetail.status === 1 ? '启用' : '禁用' }}
              </el-tag>
            </div>

            <div class="menu-management__detail-meta">
              <article>
                <span>类型</span>
                <strong>{{ resolveMenuTypeLabel(currentDetail.menu_type) }}</strong>
              </article>
              <article>
                <span>排序</span>
                <strong>{{ currentDetail.sort_no }}</strong>
              </article>
              <article>
                <span>显示</span>
                <strong>{{ formatBoolean(currentDetail.visible) }}</strong>
              </article>
            </div>
          </div>

          <section class="menu-management__detail-section menu-management__panel">
            <header class="menu-management__detail-section-header">
              <strong>路由与权限</strong>
              <span>确认当前菜单节点对应的前端路由、组件路径和权限标识。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="上级菜单 ID">
                {{ currentDetail.parent_id || '根目录' }}
              </el-descriptions-item>
              <el-descriptions-item label="路由名称">
                {{ currentDetail.route_name ?? '未配置' }}
              </el-descriptions-item>
              <el-descriptions-item label="路由路径">
                {{ currentDetail.route_path ?? '未配置' }}
              </el-descriptions-item>
              <el-descriptions-item label="组件路径">
                {{ currentDetail.component_path ?? '未配置' }}
              </el-descriptions-item>
              <el-descriptions-item label="权限标识">
                {{ currentDetail.permission_code ?? '未绑定' }}
              </el-descriptions-item>
              <el-descriptions-item label="图标">
                {{ currentDetail.icon ?? '未配置' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="menu-management__detail-section menu-management__panel">
            <header class="menu-management__detail-section-header">
              <strong>行为配置</strong>
              <span>这些字段会影响菜单显示、缓存和外链行为。</span>
            </header>

            <el-descriptions
              :column="1"
              border
            >
              <el-descriptions-item label="是否显示">
                {{ formatBoolean(currentDetail.visible) }}
              </el-descriptions-item>
              <el-descriptions-item label="是否缓存">
                {{ formatBoolean(currentDetail.keep_alive) }}
              </el-descriptions-item>
              <el-descriptions-item label="是否外链">
                {{ formatBoolean(currentDetail.is_external) }}
              </el-descriptions-item>
              <el-descriptions-item label="备注">
                {{ currentDetail.remark ?? '无' }}
              </el-descriptions-item>
            </el-descriptions>
          </section>

          <section class="menu-management__detail-section menu-management__panel">
            <header class="menu-management__detail-section-header">
              <strong>审计信息</strong>
              <span>用于确认菜单创建和最近更新时间。</span>
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
      width="820px"
      @cancel="resetForm"
    >
      <div
        v-loading="modalLoading"
        class="menu-management__modal-body"
      >
        <el-form
          ref="formRef"
          :model="form"
          :rules="formRules"
          label-position="top"
          class="menu-management__form"
        >
          <section class="menu-management__form-section menu-management__panel">
            <strong>基础信息</strong>
            <span>定义菜单层级、类型、名称和排序。</span>

            <div class="menu-management__form-grid">
              <el-form-item label="上级菜单">
                <el-select
                  v-model="form.parentId"
                  filterable
                >
                  <el-option
                    v-for="option in parentOptions"
                    :key="option.id"
                    :label="option.label"
                    :value="option.id"
                    :disabled="option.disabled"
                  />
                </el-select>
              </el-form-item>

              <el-form-item
                label="菜单类型"
                prop="menuType"
              >
                <el-select v-model="form.menuType">
                  <el-option
                    v-for="option in menuTypeOptions"
                    :key="option.value"
                    :label="option.label"
                    :value="option.value"
                  >
                    <span>{{ option.label }}</span>
                    <span class="menu-management__option-desc">{{ option.description }}</span>
                  </el-option>
                </el-select>
              </el-form-item>

              <el-form-item
                label="菜单名称"
                prop="menuName"
              >
                <el-input
                  v-model="form.menuName"
                  placeholder="请输入菜单名称"
                />
              </el-form-item>

              <el-form-item label="排序">
                <el-input-number
                  v-model="form.sortNo"
                  :min="0"
                  :max="9999"
                  controls-position="right"
                  class="menu-management__number-input"
                />
              </el-form-item>
            </div>
          </section>

          <section class="menu-management__form-section menu-management__panel">
            <strong>路由与权限</strong>
            <span>页面菜单建议维护路由和组件路径，按钮/API 节点必须维护权限标识。</span>

            <div class="menu-management__form-grid">
              <el-form-item label="路由名称">
                <el-input
                  v-model="form.routeName"
                  placeholder="例如 SystemMenu"
                />
              </el-form-item>

              <el-form-item
                label="路由路径"
                prop="routePath"
              >
                <el-input
                  v-model="form.routePath"
                  placeholder="例如 /system/menu"
                />
              </el-form-item>

              <el-form-item label="组件路径">
                <el-input
                  v-model="form.componentPath"
                  placeholder="例如 system/menu/index"
                />
              </el-form-item>

              <el-form-item
                label="权限标识"
                prop="permissionCode"
              >
                <el-input
                  v-model="form.permissionCode"
                  placeholder="例如 system:menu:list"
                />
              </el-form-item>

              <el-form-item label="图标">
                <el-input
                  v-model="form.icon"
                  placeholder="例如 Menu"
                />
              </el-form-item>
            </div>
          </section>

          <section class="menu-management__form-section menu-management__panel">
            <strong>行为配置</strong>
            <span>控制导航显示、页面缓存、外链和节点状态。</span>

            <div class="menu-management__form-grid">
              <el-form-item label="显示到菜单">
                <el-switch
                  v-model="form.visible"
                  active-text="显示"
                  inactive-text="隐藏"
                />
              </el-form-item>

              <el-form-item label="页面缓存">
                <el-switch
                  v-model="form.keepAlive"
                  active-text="缓存"
                  inactive-text="不缓存"
                />
              </el-form-item>

              <el-form-item label="外链">
                <el-switch
                  v-model="form.isExternal"
                  active-text="外链"
                  inactive-text="内部路由"
                />
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
                class="menu-management__form-span-2"
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
          {{ isCreateMode ? '创建菜单' : '保存修改' }}
        </el-button>
      </template>
    </PageModal>
  </PageContent>
</template>

<style scoped>
.menu-management__overview {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.menu-management__overview-card {
  display: grid;
  gap: 8px;
  padding: 18px;
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-lg);
  background: var(--app-surface-strong);
  position: relative;
  overflow: hidden;
}

.menu-management__overview-card::before {
  content: '';
  position: absolute;
  inset: 0 auto 0 0;
  width: 4px;
  background: var(--app-primary);
}

.menu-management__overview-card--success::before {
  background: var(--app-success);
}

.menu-management__overview-card--warning::before {
  background: var(--app-warning);
}

.menu-management__overview-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.menu-management__overview-head em {
  padding: 4px 10px;
  border-radius: 999px;
  background: var(--app-surface-soft);
  color: var(--app-text-soft);
  font-size: 11px;
  font-style: normal;
  font-weight: 700;
}

.menu-management__overview-card--success .menu-management__overview-head em {
  background: color-mix(in srgb, var(--app-success) 10%, white);
  color: var(--app-success);
}

.menu-management__overview-card--warning .menu-management__overview-head em {
  background: color-mix(in srgb, var(--app-warning) 14%, white);
  color: #b45309;
}

.menu-management__overview-card span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.menu-management__overview-card strong {
  color: #111827;
  font-size: 28px;
  line-height: 1;
}

.menu-management__overview-card p {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 13px;
  line-height: 1.7;
}

.menu-management__search-form {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.menu-management__search-shell {
  display: grid;
  gap: 14px;
}

.menu-management__section-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.menu-management__section-copy {
  display: grid;
  gap: 4px;
}

.menu-management__section-copy strong {
  color: #111827;
  font-size: 15px;
}

.menu-management__section-copy span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.menu-management__section-side {
  display: flex;
  align-items: center;
}

.menu-management__section-badge {
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

.menu-management__section-badge--active {
  border-color: color-mix(in srgb, var(--app-primary) 18%, white);
  background: var(--app-primary-soft);
  color: var(--app-primary);
}

.menu-management__search-actions {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.menu-management__search-summary {
  margin: 0;
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.menu-management__search-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.menu-management__search-form :deep(.el-form-item) {
  margin-bottom: 0;
}

.menu-management__table-toolbar {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
}

.menu-management__table-meta {
  display: grid;
  gap: 4px;
}

.menu-management__table-meta strong {
  color: #111827;
  font-size: 15px;
}

.menu-management__table-meta span {
  color: var(--app-text-soft);
  font-size: 12px;
}

.menu-management__table-pills {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.menu-management__table-pill {
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

.menu-management__table-pill--success {
  background: #edf9f1;
  color: #1f8f55;
}

.menu-management__table-pill--danger {
  background: #fdf0f0;
  color: #c24141;
}

.menu-management__row-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: nowrap;
}

.menu-management__row-action {
  margin-left: 0;
  justify-content: flex-start;
  padding: 2px 0;
  flex-shrink: 0;
}

.menu-management__panel {
  border: 1px solid var(--app-border);
  border-radius: var(--app-radius-md);
  background: var(--app-surface-strong);
}

.menu-management__detail {
  display: grid;
  gap: 16px;
  min-height: 240px;
}

.menu-management__detail-hero-shell {
  display: grid;
  gap: 16px;
  padding: 18px;
}

.menu-management__detail-hero {
  display: flex;
  align-items: center;
  gap: 12px;
}

.menu-management__detail-avatar {
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

.menu-management__detail-copy {
  display: grid;
  gap: 4px;
  flex: 1;
}

.menu-management__detail-copy strong {
  color: #111827;
  font-size: 18px;
}

.menu-management__detail-copy span {
  color: var(--app-text-soft);
  font-size: 13px;
}

.menu-management__detail-meta {
  display: grid;
  gap: 10px;
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.menu-management__detail-meta article {
  display: grid;
  gap: 6px;
  padding: 14px;
  border-radius: 14px;
  background: var(--app-surface-soft);
}

.menu-management__detail-meta article span {
  color: var(--app-text-faint);
  font-size: 12px;
}

.menu-management__detail-meta article strong {
  color: #111827;
  font-size: 14px;
  line-height: 1.6;
}

.menu-management__detail-section {
  display: grid;
  gap: 10px;
  padding: 18px;
}

.menu-management__detail-section-header {
  display: grid;
  gap: 4px;
}

.menu-management__detail-section-header strong {
  color: #111827;
  font-size: 14px;
}

.menu-management__detail-section-header span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.menu-management__modal-body {
  min-height: 220px;
}

.menu-management__form {
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr;
}

.menu-management__form-section {
  display: grid;
  gap: 14px;
  padding: 18px;
}

.menu-management__form-section strong {
  color: #111827;
  font-size: 14px;
}

.menu-management__form-section span {
  color: var(--app-text-soft);
  font-size: 12px;
  line-height: 1.6;
}

.menu-management__form-grid {
  display: grid;
  gap: 12px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.menu-management__form-span-2 {
  grid-column: 1 / -1;
}

.menu-management__number-input {
  width: 100%;
}

.menu-management__option-desc {
  float: right;
  color: var(--app-text-faint);
  font-size: 12px;
}

.menu-management__form :deep(.el-form-item) {
  margin-bottom: 0;
}

@media (max-width: 1080px) {
  .menu-management__overview,
  .menu-management__search-form,
  .menu-management__form-grid {
    grid-template-columns: 1fr;
  }

  .menu-management__search-actions {
    align-items: flex-start;
  }

  .menu-management__detail-meta {
    grid-template-columns: 1fr;
  }

  .menu-management__form-span-2 {
    grid-column: auto;
  }
}
</style>
