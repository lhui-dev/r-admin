import request from '@/utils/request'

interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface UserDeptSummary {
  id: number
  name: string
}

export interface UserRoleSummary {
  id: number
  code: string
  name: string
}

export interface UserPostSummary {
  id: number
  code: string
  name: string
}

export interface UserListItem {
  id: number
  username: string
  nickname: string
  real_name: string | null
  mobile: string | null
  email: string | null
  status: number
  is_super_admin: boolean
  dept: UserDeptSummary | null
  roles: UserRoleSummary[]
  posts: UserPostSummary[]
  last_login_at: string | null
  created_at: string
}

export interface UserListResponseData {
  items: UserListItem[]
  pagination: {
    page: number
    page_size: number
    total: number
  }
}

export interface UserDetailData {
  id: number
  username: string
  nickname: string
  real_name: string | null
  mobile: string | null
  email: string | null
  avatar_url: string | null
  gender: number | null
  status: number
  is_super_admin: boolean
  remark: string | null
  dept: UserDeptSummary | null
  roles: UserRoleSummary[]
  posts: UserPostSummary[]
  last_login_at: string | null
  last_login_ip: string | null
  password_updated_at: string | null
  created_at: string
  updated_at: string
}

export interface UserListQuery {
  page?: number
  page_size?: number
  keyword?: string
  dept_id?: number
  status?: number
}

export interface CreateUserPayload {
  username: string
  password: string
  nickname: string
  real_name?: string
  mobile?: string
  email?: string
  gender?: number
  dept_id?: number
  status?: number
  role_ids?: number[]
  remark?: string
}

export interface UpdateUserPayload {
  nickname?: string
  real_name?: string
  mobile?: string
  email?: string
  gender?: number
  dept_id?: number
  role_ids?: number[]
  remark?: string
}

export interface UserMutationData {
  id: number
  username: string
}

export interface UserStatusMutationData extends UserMutationData {
  status: number
}

export interface UpdateUserRolesPayload {
  role_ids: number[]
}

export function fetchSystemUsers(params: UserListQuery) {
  return request.get<ApiResponse<UserListResponseData>, ApiResponse<UserListResponseData>>(
    '/system/users',
    { params },
  )
}

export function fetchSystemUserDetail(userId: number) {
  return request.get<ApiResponse<UserDetailData>, ApiResponse<UserDetailData>>(`/system/users/${userId}`)
}

export function createSystemUser(payload: CreateUserPayload) {
  return request.post<ApiResponse<UserMutationData>, ApiResponse<UserMutationData>>('/system/users', payload)
}

export function updateSystemUser(userId: number, payload: UpdateUserPayload) {
  return request.patch<ApiResponse<UserMutationData>, ApiResponse<UserMutationData>>(`/system/users/${userId}`, payload)
}

export function updateSystemUserStatus(userId: number, status: number) {
  return request.patch<ApiResponse<UserStatusMutationData>, ApiResponse<UserStatusMutationData>>(
    `/system/users/${userId}/status`,
    { status },
  )
}

export function updateSystemUserRoles(userId: number, payload: UpdateUserRolesPayload) {
  return request.patch<ApiResponse<UserMutationData>, ApiResponse<UserMutationData>>(
    `/system/users/${userId}/roles`,
    payload,
  )
}
