interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface RolePermissionSummary {
  id: string
  name: string
}

export interface RolePermissionTreeNode {
  id: string
  name: string
  type: 'module' | 'menu' | 'button' | 'api'
  children?: RolePermissionTreeNode[]
}

export interface RoleListItem {
  id: number
  code: string
  name: string
  status: number
  data_scope: string
  sort: number
  is_builtin: boolean
  user_count: number
  permission_count: number
  remark: string | null
  created_at: string
}

export interface RoleDetailData extends RoleListItem {
  updated_at: string
  permissions: RolePermissionSummary[]
}

export interface RoleListResponseData {
  items: RoleListItem[]
  pagination: {
    page: number
    page_size: number
    total: number
  }
}

export interface RoleListQuery {
  page?: number
  page_size?: number
  keyword?: string
  status?: number
  data_scope?: string
}

export interface CreateRolePayload {
  code: string
  name: string
  status?: number
  data_scope: string
  sort?: number
  remark?: string
}

export interface UpdateRolePayload {
  name?: string
  status?: number
  data_scope?: string
  sort?: number
  remark?: string
}

export interface RoleMutationData {
  id: number
  code: string
  name: string
}

export interface RoleStatusMutationData extends RoleMutationData {
  status: number
}

export interface RolePermissionConfigData {
  role: RoleDetailData
  permission_tree: RolePermissionTreeNode[]
  checked_permission_ids: string[]
}

export interface UpdateRolePermissionsPayload {
  permission_ids: string[]
}

type InternalRoleRecord = RoleDetailData

const roleStore: InternalRoleRecord[] = [
  {
    id: 1,
    code: 'super_admin',
    name: '超级管理员',
    status: 1,
    data_scope: 'all',
    sort: 1,
    is_builtin: true,
    user_count: 2,
    permission_count: 48,
    remark: '拥有系统全部菜单与操作权限。',
    created_at: '2026-05-01 09:00:00',
    updated_at: '2026-06-03 10:20:00',
    permissions: [
      { id: 'dashboard:view', name: '工作台查看' },
      { id: 'system:user:list', name: '用户管理' },
      { id: 'system:role:list', name: '角色管理' },
      { id: 'system:menu:list', name: '菜单管理' },
    ],
  },
  {
    id: 2,
    code: 'tenant_admin',
    name: '租户管理员',
    status: 1,
    data_scope: 'tenant',
    sort: 5,
    is_builtin: true,
    user_count: 6,
    permission_count: 26,
    remark: '负责租户内用户、角色和菜单的基础维护。',
    created_at: '2026-05-03 11:30:00',
    updated_at: '2026-06-02 16:10:00',
    permissions: [
      { id: 'system:user:list', name: '用户管理' },
      { id: 'system:role:list', name: '角色管理' },
      { id: 'system:dept:list', name: '部门管理' },
    ],
  },
  {
    id: 3,
    code: 'security_auditor',
    name: '安全审计员',
    status: 1,
    data_scope: 'custom',
    sort: 20,
    is_builtin: false,
    user_count: 3,
    permission_count: 12,
    remark: '查看登录日志、操作日志与关键配置变更记录。',
    created_at: '2026-05-12 14:20:00',
    updated_at: '2026-06-01 18:30:00',
    permissions: [
      { id: 'system:log:login:list', name: '登录日志查看' },
      { id: 'system:log:operation:list', name: '操作日志查看' },
      { id: 'audit:center:view', name: '审计中心查看' },
    ],
  },
  {
    id: 4,
    code: 'ops_manager',
    name: '运营经理',
    status: 0,
    data_scope: 'department',
    sort: 30,
    is_builtin: false,
    user_count: 4,
    permission_count: 9,
    remark: '用于运营团队成员管理和基础数据查看。',
    created_at: '2026-05-15 09:45:00',
    updated_at: '2026-05-28 12:05:00',
    permissions: [
      { id: 'dashboard:view', name: '工作台查看' },
      { id: 'system:user:list', name: '用户管理' },
      { id: 'system:dept:list', name: '部门管理' },
    ],
  },
  {
    id: 5,
    code: 'readonly_guest',
    name: '只读访客',
    status: 1,
    data_scope: 'self',
    sort: 50,
    is_builtin: false,
    user_count: 8,
    permission_count: 4,
    remark: '仅允许访问个人资料与部分概览信息。',
    created_at: '2026-05-18 13:15:00',
    updated_at: '2026-05-31 17:20:00',
    permissions: [
      { id: 'dashboard:view', name: '工作台查看' },
      { id: 'profile:view', name: '个人资料查看' },
    ],
  },
]

const permissionTreeStore: RolePermissionTreeNode[] = [
  {
    id: 'module:dashboard',
    name: '工作台',
    type: 'module',
    children: [
      {
        id: 'menu:dashboard:view',
        name: '工作台页面',
        type: 'menu',
        children: [
          { id: 'dashboard:view', name: '查看工作台', type: 'button' },
          { id: 'dashboard:refresh', name: '刷新看板', type: 'button' },
        ],
      },
    ],
  },
  {
    id: 'module:system',
    name: '系统管理',
    type: 'module',
    children: [
      {
        id: 'menu:system:user',
        name: '用户管理',
        type: 'menu',
        children: [
          { id: 'system:user:list', name: '查看用户列表', type: 'button' },
          { id: 'system:user:create', name: '新增用户', type: 'button' },
          { id: 'system:user:update', name: '编辑用户', type: 'button' },
          { id: 'system:user:status', name: '启停用户', type: 'button' },
        ],
      },
      {
        id: 'menu:system:role',
        name: '角色管理',
        type: 'menu',
        children: [
          { id: 'system:role:list', name: '查看角色列表', type: 'button' },
          { id: 'system:role:create', name: '新增角色', type: 'button' },
          { id: 'system:role:update', name: '编辑角色', type: 'button' },
          { id: 'system:role:permission', name: '配置角色权限', type: 'button' },
        ],
      },
      {
        id: 'menu:system:menu',
        name: '菜单管理',
        type: 'menu',
        children: [
          { id: 'system:menu:list', name: '查看菜单列表', type: 'button' },
          { id: 'system:menu:update', name: '编辑菜单', type: 'button' },
        ],
      },
      {
        id: 'menu:system:dept',
        name: '部门管理',
        type: 'menu',
        children: [
          { id: 'system:dept:list', name: '查看部门列表', type: 'button' },
          { id: 'system:dept:update', name: '编辑部门', type: 'button' },
        ],
      },
    ],
  },
  {
    id: 'module:audit',
    name: '日志审计',
    type: 'module',
    children: [
      {
        id: 'menu:audit:login',
        name: '登录日志',
        type: 'menu',
        children: [
          { id: 'system:log:login:list', name: '查看登录日志', type: 'button' },
        ],
      },
      {
        id: 'menu:audit:operation',
        name: '操作日志',
        type: 'menu',
        children: [
          { id: 'system:log:operation:list', name: '查看操作日志', type: 'button' },
          { id: 'audit:center:view', name: '查看审计中心', type: 'button' },
        ],
      },
    ],
  },
  {
    id: 'module:profile',
    name: '个人中心',
    type: 'module',
    children: [
      {
        id: 'menu:profile:view',
        name: '个人信息',
        type: 'menu',
        children: [
          { id: 'profile:view', name: '查看个人信息', type: 'button' },
        ],
      },
    ],
  },
]

// Frontend prototype phase: keep role CRUD runnable with local mock data
// until backend role endpoints are ready, while preserving the final API shape.
export async function fetchSystemRoles(params: RoleListQuery) {
  await sleep(120)

  const page = params.page ?? 1
  const pageSize = params.page_size ?? 10
  const keyword = params.keyword?.trim().toLowerCase()

  let filtered = [...roleStore]

  if (keyword) {
    filtered = filtered.filter(role =>
      [role.code, role.name, role.remark ?? '']
        .some(field => field.toLowerCase().includes(keyword)),
    )
  }

  if (params.status !== undefined) {
    filtered = filtered.filter(role => role.status === params.status)
  }

  if (params.data_scope) {
    filtered = filtered.filter(role => role.data_scope === params.data_scope)
  }

  filtered.sort((a, b) => a.sort - b.sort || a.id - b.id)

  const start = (page - 1) * pageSize
  const items = filtered.slice(start, start + pageSize).map(toRoleListItem)

  return buildResponse<RoleListResponseData>({
    items,
    pagination: {
      page,
      page_size: pageSize,
      total: filtered.length,
    },
  })
}

export async function fetchSystemRoleDetail(roleId: number) {
  await sleep(120)

  const role = roleStore.find(item => item.id === roleId)
  if (!role) {
    throw new Error('角色详情不存在')
  }

  return buildResponse<RoleDetailData>({ ...role, permissions: [...role.permissions] })
}

export async function createSystemRole(payload: CreateRolePayload) {
  await sleep(120)

  const normalizedCode = payload.code.trim()
  if (roleStore.some(role => role.code === normalizedCode)) {
    throw new Error('角色编码已存在')
  }

  const now = currentTimestamp()
  const nextId = Math.max(...roleStore.map(role => role.id)) + 1
  const nextRole: InternalRoleRecord = {
    id: nextId,
    code: normalizedCode,
    name: payload.name.trim(),
    status: payload.status ?? 1,
    data_scope: payload.data_scope,
    sort: payload.sort ?? 100,
    is_builtin: false,
    user_count: 0,
    permission_count: 0,
    remark: normalizeText(payload.remark),
    created_at: now,
    updated_at: now,
    permissions: [],
  }

  roleStore.push(nextRole)

  return buildResponse<RoleMutationData>({
    id: nextRole.id,
    code: nextRole.code,
    name: nextRole.name,
  })
}

export async function updateSystemRole(roleId: number, payload: UpdateRolePayload) {
  await sleep(120)

  const role = roleStore.find(item => item.id === roleId)
  if (!role) {
    throw new Error('角色不存在，无法更新')
  }

  role.name = normalizeText(payload.name) ?? role.name
  role.data_scope = payload.data_scope ?? role.data_scope
  role.sort = payload.sort ?? role.sort
  role.remark = payload.remark !== undefined ? normalizeText(payload.remark) : role.remark
  role.updated_at = currentTimestamp()

  return buildResponse<RoleMutationData>({
    id: role.id,
    code: role.code,
    name: role.name,
  })
}

export async function updateSystemRoleStatus(roleId: number, status: number) {
  await sleep(120)

  const role = roleStore.find(item => item.id === roleId)
  if (!role) {
    throw new Error('角色不存在，无法变更状态')
  }

  role.status = status
  role.updated_at = currentTimestamp()

  return buildResponse<RoleStatusMutationData>({
    id: role.id,
    code: role.code,
    name: role.name,
    status: role.status,
  })
}

export async function fetchSystemRolePermissionConfig(roleId: number) {
  await sleep(120)

  const role = roleStore.find(item => item.id === roleId)
  if (!role) {
    throw new Error('角色不存在，无法加载权限配置')
  }

  return buildResponse<RolePermissionConfigData>({
    role: { ...role, permissions: [...role.permissions] },
    permission_tree: clonePermissionTree(permissionTreeStore),
    checked_permission_ids: role.permissions.map(item => item.id),
  })
}

export async function updateSystemRolePermissions(roleId: number, payload: UpdateRolePermissionsPayload) {
  await sleep(120)

  const role = roleStore.find(item => item.id === roleId)
  if (!role) {
    throw new Error('角色不存在，无法保存权限配置')
  }

  const permissionMap = new Map(flattenPermissionLeaves(permissionTreeStore).map(item => [item.id, item.name]))
  const nextPermissions = payload.permission_ids
    .filter(permissionId => permissionMap.has(permissionId))
    .map(permissionId => ({
      id: permissionId,
      name: permissionMap.get(permissionId) ?? permissionId,
    }))

  role.permissions = nextPermissions
  role.permission_count = nextPermissions.length
  role.updated_at = currentTimestamp()

  return buildResponse<RoleDetailData>({
    ...role,
    permissions: [...role.permissions],
  })
}

function toRoleListItem(role: InternalRoleRecord): RoleListItem {
  return {
    id: role.id,
    code: role.code,
    name: role.name,
    status: role.status,
    data_scope: role.data_scope,
    sort: role.sort,
    is_builtin: role.is_builtin,
    user_count: role.user_count,
    permission_count: role.permission_count,
    remark: role.remark,
    created_at: role.created_at,
  }
}

function buildResponse<T>(data: T): ApiResponse<T> {
  return {
    code: 0,
    message: 'ok',
    data,
  }
}

function normalizeText(value?: string) {
  const trimmed = value?.trim()
  return trimmed ? trimmed : null
}

function currentTimestamp() {
  const now = new Date()
  const yyyy = now.getFullYear()
  const mm = String(now.getMonth() + 1).padStart(2, '0')
  const dd = String(now.getDate()).padStart(2, '0')
  const hh = String(now.getHours()).padStart(2, '0')
  const mi = String(now.getMinutes()).padStart(2, '0')
  const ss = String(now.getSeconds()).padStart(2, '0')
  return `${yyyy}-${mm}-${dd} ${hh}:${mi}:${ss}`
}

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

function clonePermissionTree(tree: RolePermissionTreeNode[]): RolePermissionTreeNode[] {
  return tree.map(node => ({
    ...node,
    children: node.children ? clonePermissionTree(node.children) : undefined,
  }))
}

function flattenPermissionLeaves(tree: RolePermissionTreeNode[]): Array<{ id: string, name: string }> {
  return tree.flatMap((node) => {
    if (!node.children?.length) {
      return node.type === 'button' || node.type === 'api'
        ? [{ id: node.id, name: node.name }]
        : []
    }

    return flattenPermissionLeaves(node.children)
  })
}
