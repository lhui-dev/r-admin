import request from '@/utils/request'

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
    id: 300,
    code: 'super_admin',
    name: '超级管理员',
    status: 1,
    data_scope: 'all',
    sort: 1,
    is_builtin: true,
    user_count: 1,
    permission_count: 34,
    remark: '拥有系统全部菜单与操作权限。',
    created_at: '2026-05-31T14:43:34Z',
    updated_at: '2026-06-03T14:43:34Z',
    permissions: [
      { id: 'dashboard:view', name: '首页查看' },
      { id: 'system:user:list', name: '用户列表' },
      { id: 'system:role:list', name: '角色列表' },
      { id: 'system:menu:list', name: '菜单列表' },
    ],
  },
  {
    id: 310,
    code: 'system_admin',
    name: '系统管理员',
    status: 1,
    data_scope: 'department',
    sort: 10,
    is_builtin: true,
    user_count: 1,
    permission_count: 27,
    remark: '系统内置系统管理员角色',
    created_at: '2026-05-31T14:43:34Z',
    updated_at: '2026-06-03T14:48:16Z',
    permissions: [
      { id: 'system:user:list', name: '用户列表' },
      { id: 'system:role:list', name: '角色列表' },
      { id: 'system:role:assign-permission', name: '角色分配权限' },
    ],
  },
  {
    id: 320,
    code: 'auditor',
    name: '审计员',
    status: 1,
    data_scope: 'all',
    sort: 20,
    is_builtin: true,
    user_count: 1,
    permission_count: 3,
    remark: '系统内置审计角色',
    created_at: '2026-05-31T14:43:34Z',
    updated_at: '2026-06-03T14:43:34Z',
    permissions: [
      { id: 'dashboard:view', name: '首页查看' },
      { id: 'system:log:login:list', name: '登录日志列表' },
      { id: 'system:log:operation:list', name: '操作日志列表' },
    ],
  },
  {
    id: 330,
    code: 'operator',
    name: '普通操作员',
    status: 1,
    data_scope: 'self',
    sort: 30,
    is_builtin: true,
    user_count: 0,
    permission_count: 1,
    remark: '系统内置普通操作角色',
    created_at: '2026-05-31T14:43:34Z',
    updated_at: '2026-06-03T14:43:34Z',
    permissions: [
      { id: 'dashboard:view', name: '首页查看' },
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
          { id: 'dashboard:view', name: '首页查看', type: 'button' },
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
          { id: 'system:user:list', name: '用户列表', type: 'button' },
          { id: 'system:user:create', name: '用户新增', type: 'button' },
          { id: 'system:user:update', name: '用户修改', type: 'button' },
          { id: 'system:user:delete', name: '用户删除', type: 'button' },
          { id: 'system:user:reset-password', name: '用户重置密码', type: 'button' },
          { id: 'system:user:assign-role', name: '用户分配角色', type: 'button' },
        ],
      },
      {
        id: 'menu:system:role',
        name: '角色管理',
        type: 'menu',
        children: [
          { id: 'system:role:list', name: '角色列表', type: 'button' },
          { id: 'system:role:create', name: '角色新增', type: 'button' },
          { id: 'system:role:update', name: '角色修改', type: 'button' },
          { id: 'system:role:delete', name: '角色删除', type: 'button' },
          { id: 'system:role:assign-permission', name: '角色分配权限', type: 'button' },
        ],
      },
      {
        id: 'menu:system:menu',
        name: '菜单管理',
        type: 'menu',
        children: [
          { id: 'system:menu:list', name: '菜单列表', type: 'button' },
          { id: 'system:menu:create', name: '菜单新增', type: 'button' },
          { id: 'system:menu:update', name: '菜单修改', type: 'button' },
          { id: 'system:menu:delete', name: '菜单删除', type: 'button' },
        ],
      },
      {
        id: 'menu:system:dept',
        name: '部门管理',
        type: 'menu',
        children: [
          { id: 'system:dept:list', name: '部门列表', type: 'button' },
          { id: 'system:dept:create', name: '部门新增', type: 'button' },
          { id: 'system:dept:update', name: '部门修改', type: 'button' },
          { id: 'system:dept:delete', name: '部门删除', type: 'button' },
        ],
      },
      {
        id: 'menu:system:post',
        name: '岗位管理',
        type: 'menu',
        children: [
          { id: 'system:post:list', name: '岗位列表', type: 'button' },
          { id: 'system:post:create', name: '岗位新增', type: 'button' },
          { id: 'system:post:update', name: '岗位修改', type: 'button' },
          { id: 'system:post:delete', name: '岗位删除', type: 'button' },
        ],
      },
      {
        id: 'menu:system:dict',
        name: '字典管理',
        type: 'menu',
        children: [
          { id: 'system:dict:list', name: '字典列表', type: 'button' },
          { id: 'system:dict:create', name: '字典新增', type: 'button' },
          { id: 'system:dict:update', name: '字典修改', type: 'button' },
          { id: 'system:dict:delete', name: '字典删除', type: 'button' },
        ],
      },
      {
        id: 'menu:system:config',
        name: '参数配置',
        type: 'menu',
        children: [
          { id: 'system:config:list', name: '参数列表', type: 'button' },
          { id: 'system:config:create', name: '参数新增', type: 'button' },
          { id: 'system:config:update', name: '参数修改', type: 'button' },
          { id: 'system:config:delete', name: '参数删除', type: 'button' },
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
          { id: 'system:log:login:list', name: '登录日志列表', type: 'button' },
        ],
      },
      {
        id: 'menu:audit:operation',
        name: '操作日志',
        type: 'menu',
        children: [
          { id: 'system:log:operation:list', name: '操作日志列表', type: 'button' },
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

  const role = findLocalRole(roleId)
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

  const role = findLocalRole(roleId)
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

  const role = findLocalRole(roleId)
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
  try {
    const response = await request.get<ApiResponse<RolePermissionConfigData>, ApiResponse<RolePermissionConfigData>>(
      `/system/roles/${roleId}/permission-config`,
    )

    const mergedRole = mergeRoleDetailWithLocal(
      roleId,
      response.data.role,
      response.data.checked_permission_ids,
    )

    syncLocalRole(mergedRole)

    return buildResponse<RolePermissionConfigData>({
      ...response.data,
      role: mergedRole,
    })
  }
  catch (error) {
    if (!shouldFallbackToMock(error)) {
      throw error
    }
  }

  await sleep(120)

  const role = findLocalRole(roleId)
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
  try {
    const response = await request.put<ApiResponse<RoleDetailData>, ApiResponse<RoleDetailData>>(
      `/system/roles/${roleId}/permissions`,
      payload,
    )

    const mergedRole = mergeRoleDetailWithLocal(
      roleId,
      response.data,
      response.data.permissions?.map(item => item.id) ?? [],
    )

    syncLocalRole(mergedRole)

    return buildResponse<RoleDetailData>(mergedRole)
  }
  catch (error) {
    if (!shouldFallbackToMock(error)) {
      throw error
    }
  }

  await sleep(120)

  const role = findLocalRole(roleId)
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

function findLocalRole(roleId: number) {
  return roleStore.find(item => item.id === roleId)
}

function syncLocalRole(nextRole: RoleDetailData) {
  const target = findLocalRole(nextRole.id)
  if (!target) {
    roleStore.push({
      ...nextRole,
      permissions: [...nextRole.permissions],
    })
    return
  }

  Object.assign(target, {
    ...nextRole,
    permissions: [...nextRole.permissions],
  })
}

function mergeRoleDetailWithLocal(
  roleId: number,
  remoteRole: RoleDetailData,
  checkedPermissionIds: string[],
): RoleDetailData {
  const localRole = findLocalRole(roleId)
  const fallbackPermissions = checkedPermissionIds.map(permissionId => ({
    id: permissionId,
    name: resolvePermissionName(permissionId),
  }))
  const mergedPermissions = Array.isArray(remoteRole.permissions) && remoteRole.permissions.length
    ? [...remoteRole.permissions]
    : [...(localRole?.permissions ?? fallbackPermissions)]

  return {
    id: remoteRole.id,
    code: localRole?.code ?? remoteRole.code,
    name: localRole?.name ?? remoteRole.name,
    status: localRole?.status ?? remoteRole.status,
    data_scope: localRole?.data_scope ?? remoteRole.data_scope,
    sort: localRole?.sort ?? remoteRole.sort,
    is_builtin: localRole?.is_builtin ?? remoteRole.is_builtin,
    user_count: remoteRole.user_count ?? localRole?.user_count ?? 0,
    permission_count: remoteRole.permission_count ?? mergedPermissions.length,
    remark: localRole?.remark ?? remoteRole.remark ?? null,
    created_at: localRole?.created_at ?? remoteRole.created_at,
    updated_at: localRole?.updated_at ?? remoteRole.updated_at,
    permissions: mergedPermissions,
  }
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

function shouldFallbackToMock(error: unknown) {
  if (typeof error !== 'object' || error === null) {
    return false
  }

  const record = error as {
    code?: unknown
    message?: unknown
    response?: { status?: unknown }
  }
  const responseCode = Number(record.code ?? Number.NaN)
  const responseStatus = Number(record.response?.status ?? Number.NaN)
  const message = String(record.message ?? '')

  return responseCode === 404
    || responseStatus === 404
    || message.includes('Network Error')
    || message.includes('timeout')
    || message.includes('ERR_NETWORK')
}

function clonePermissionTree(tree: RolePermissionTreeNode[]): RolePermissionTreeNode[] {
  return tree.map(node => ({
    ...node,
    children: node.children ? clonePermissionTree(node.children) : undefined,
  }))
}

function resolvePermissionName(permissionId: string) {
  return flattenPermissionLeaves(permissionTreeStore)
    .find(item => item.id === permissionId)?.name ?? permissionId
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
