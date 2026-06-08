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

export async function fetchSystemRoles(params: RoleListQuery) {
  return request.get<ApiResponse<RoleListResponseData>, ApiResponse<RoleListResponseData>>(
    '/system/roles',
    {params},
  )
}

export async function fetchSystemRoleDetail(roleId: number) {
  return request.get<ApiResponse<RoleDetailData>, ApiResponse<RoleDetailData>>(
    `/system/roles/${roleId}`,
  )
}

export async function createSystemRole(payload: CreateRolePayload) {
  return request.post<ApiResponse<RoleMutationData>, ApiResponse<RoleMutationData>>(
    '/system/roles',
    payload,
  )
}

export async function updateSystemRole(roleId: number, payload: UpdateRolePayload) {
  return request.patch<ApiResponse<RoleMutationData>, ApiResponse<RoleMutationData>>(
    `/system/roles/${roleId}`,
    payload,
  )
}

export async function updateSystemRoleStatus(roleId: number, status: number) {
  return request.patch<ApiResponse<RoleStatusMutationData>, ApiResponse<RoleStatusMutationData>>(
    `/system/roles/${roleId}/status`,
    {status},
  )
}

export async function fetchSystemRolePermissionConfig(roleId: number) {
  return request.get<ApiResponse<RolePermissionConfigData>, ApiResponse<RolePermissionConfigData>>(
    `/system/roles/${roleId}/permission-config`,
  )
}

export async function updateSystemRolePermissions(roleId: number, payload: UpdateRolePermissionsPayload) {
  return request.put<ApiResponse<RoleDetailData>, ApiResponse<RoleDetailData>>(
    `/system/roles/${roleId}/permissions`,
    payload,
  )
}
