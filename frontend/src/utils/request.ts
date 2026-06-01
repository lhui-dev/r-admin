import axios from 'axios'

import { getToken, removeToken } from '@/utils/auth'

type UnauthorizedHandler = () => void | Promise<void>

let unauthorizedHandler: UnauthorizedHandler | null = null

const request = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
  timeout: 10_000,
})

request.interceptors.request.use((config) => {
  const token = getToken()

  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }

  return config
})

request.interceptors.response.use(
  (response) => response.data,
  async (error) => {
    if (axios.isAxiosError(error) && error.response?.status === 401) {
      // Keep token cleanup here at the transport layer so every caller gets
      // the same logout behavior without repeating it in each API module.
      removeToken()
      await unauthorizedHandler?.()
    }

    const responseData = error.response?.data
    const hasUsableResponseData = responseData !== undefined
      && responseData !== null
      && !(typeof responseData === 'string' && responseData.trim() === '')

    return Promise.reject(hasUsableResponseData ? responseData : error)
  },
)

export function registerUnauthorizedHandler(handler: UnauthorizedHandler) {
  unauthorizedHandler = handler
}

export default request
