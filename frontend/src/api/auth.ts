import request from '@/utils/request'

export interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface AuthUserProfile {
  id: number
  username: string
  nickname: string
  real_name: string | null
  is_super_admin: boolean
}

export interface LoginPayload {
  username: string
  password: string
}

export interface LoginResponseData {
  access_token: string
  token_type: 'Bearer'
  expires_in: number
  user: AuthUserProfile
}

export interface CurrentUserResponseData {
  user: AuthUserProfile
  roles: string[]
  permissions: string[]
}

export interface LogoutResponseData {
  logged_out: boolean
}

export function login(payload: LoginPayload) {
  return request.post<ApiResponse<LoginResponseData>, ApiResponse<LoginResponseData>>(
    '/auth/login',
    payload,
  )
}

export function fetchCurrentUser() {
  return request.get<ApiResponse<CurrentUserResponseData>, ApiResponse<CurrentUserResponseData>>(
    '/auth/me',
  )
}

export function logout() {
  return request.post<ApiResponse<LogoutResponseData>, ApiResponse<LogoutResponseData>>(
    '/auth/logout',
  )
}
