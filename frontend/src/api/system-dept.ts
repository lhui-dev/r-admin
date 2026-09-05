import request from '@/utils/request'

interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface DeptTreeItem {
  id: number
  parent_id: number
  dept_name: string
  dept_code: string | null
  leader_user_id: number | null
  leader_name: string | null
  sort_no: number
  status: number
  remark: string | null
  created_at: string
  updated_at: string
  children: DeptTreeItem[]
}

export interface DeptTreeData {
  items: DeptTreeItem[]
}

export interface DeptTreeQuery {
  keyword?: string
  status?: number
}

export interface CreateDeptPayload {
  parent_id?: number
  dept_name: string
  dept_code?: string
  leader_user_id?: number
  sort_no?: number
  status?: number
  remark?: string
}

export interface UpdateDeptPayload {
  parent_id?: number
  dept_name?: string
  dept_code?: string | null
  leader_user_id?: number | null
  sort_no?: number
  status?: number
  remark?: string | null
}

export interface DeptMutationData {
  id: number
  dept_name: string
}

export interface DeptStatusMutationData {
  id: number
  dept_name: string
  status: number
}

export async function fetchSystemDeptTree(params: DeptTreeQuery) {
  return request.get<ApiResponse<DeptTreeData>, ApiResponse<DeptTreeData>>(
    '/system/depts/tree',
    { params },
  )
}

export async function fetchSystemDeptDetail(deptId: number) {
  return request.get<ApiResponse<DeptTreeItem>, ApiResponse<DeptTreeItem>>(
    `/system/depts/${deptId}`,
  )
}

export async function createSystemDept(payload: CreateDeptPayload) {
  return request.post<ApiResponse<DeptMutationData>, ApiResponse<DeptMutationData>>(
    '/system/depts',
    payload,
  )
}

export async function updateSystemDept(deptId: number, payload: UpdateDeptPayload) {
  return request.patch<ApiResponse<DeptMutationData>, ApiResponse<DeptMutationData>>(
    `/system/depts/${deptId}`,
    payload,
  )
}

export async function updateSystemDeptStatus(deptId: number, status: number) {
  return request.patch<ApiResponse<DeptStatusMutationData>, ApiResponse<DeptStatusMutationData>>(
    `/system/depts/${deptId}/status`,
    { status },
  )
}

export async function deleteSystemDept(deptId: number) {
  return request.delete<ApiResponse<DeptMutationData>, ApiResponse<DeptMutationData>>(
    `/system/depts/${deptId}`,
  )
}
