import request from '@/utils/request'

export interface HealthResponse {
  code: number
  message: string
  data: {
    status: string
    service: string
    database: string
  }
}

export function fetchHealth() {
  return request.get<HealthResponse, HealthResponse>('/health')
}
