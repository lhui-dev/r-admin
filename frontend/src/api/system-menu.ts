import request from '@/utils/request'

interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export type MenuType = 'catalog' | 'menu' | 'button' | 'api'

export interface MenuTreeItem {
  id: number
  parent_id: number
  menu_name: string
  menu_type: MenuType
  route_name: string | null
  route_path: string | null
  component_path: string | null
  permission_code: string | null
  icon: string | null
  sort_no: number
  visible: boolean
  keep_alive: boolean
  is_external: boolean
  status: number
  remark: string | null
  created_at: string
  updated_at: string
  children: MenuTreeItem[]
}

export interface MenuTreeData {
  items: MenuTreeItem[]
}

export interface MenuTreeQuery {
  keyword?: string
  status?: number
  menu_type?: MenuType
}

export interface CreateMenuPayload {
  parent_id?: number
  menu_name: string
  menu_type: MenuType
  route_name?: string
  route_path?: string
  component_path?: string
  permission_code?: string
  icon?: string
  sort_no?: number
  visible?: boolean
  keep_alive?: boolean
  is_external?: boolean
  status?: number
  remark?: string
}

export type UpdateMenuPayload = Partial<CreateMenuPayload>

export interface MenuMutationData {
  id: number
  menu_name: string
  menu_type: MenuType
}

export interface MenuStatusMutationData {
  id: number
  menu_name: string
  status: number
}

export async function fetchSystemMenuTree(params: MenuTreeQuery) {
  return request.get<ApiResponse<MenuTreeData>, ApiResponse<MenuTreeData>>(
    '/system/menus/tree',
    { params },
  )
}

export async function fetchSystemMenuDetail(menuId: number) {
  return request.get<ApiResponse<MenuTreeItem>, ApiResponse<MenuTreeItem>>(
    `/system/menus/${menuId}`,
  )
}

export async function createSystemMenu(payload: CreateMenuPayload) {
  return request.post<ApiResponse<MenuMutationData>, ApiResponse<MenuMutationData>>(
    '/system/menus',
    payload,
  )
}

export async function updateSystemMenu(menuId: number, payload: UpdateMenuPayload) {
  return request.patch<ApiResponse<MenuMutationData>, ApiResponse<MenuMutationData>>(
    `/system/menus/${menuId}`,
    payload,
  )
}

export async function updateSystemMenuStatus(menuId: number, status: number) {
  return request.patch<ApiResponse<MenuStatusMutationData>, ApiResponse<MenuStatusMutationData>>(
    `/system/menus/${menuId}/status`,
    { status },
  )
}

export async function deleteSystemMenu(menuId: number) {
  return request.delete<ApiResponse<MenuMutationData>, ApiResponse<MenuMutationData>>(
    `/system/menus/${menuId}`,
  )
}
