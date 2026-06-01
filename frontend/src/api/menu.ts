import request from '@/utils/request'
import type { AppMenuItem } from '@/types/menu'

interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface CurrentMenusResponseData {
  menus: AppMenuItem[]
}

export function fetchCurrentMenus() {
  return request.get<ApiResponse<CurrentMenusResponseData>, ApiResponse<CurrentMenusResponseData>>(
    '/auth/menus',
  )
}
