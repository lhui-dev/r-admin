import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import {
  fetchCurrentUser,
  login as loginRequest,
  logout as logoutRequest,
  type AuthUserProfile,
  type CurrentUserResponseData,
  type LoginPayload,
  type LoginResponseData,
} from '@/api/auth'
import { getToken, removeToken, setToken } from '@/utils/auth'

export const useAuthStore = defineStore('auth', () => {
  const accessToken = ref(getToken())
  const currentUser = ref<AuthUserProfile | null>(null)
  const roles = ref<string[]>([])
  const permissions = ref<string[]>([])
  const profileLoaded = ref(false)
  // Reuse the same /me request during bootstrap bursts to avoid duplicate
  // network calls from route guards and page initialization racing together.
  const profileRequest = ref<Promise<CurrentUserResponseData> | null>(null)

  const hasToken = computed(() => Boolean(accessToken.value))
  const isAuthenticated = computed(() => hasToken.value && currentUser.value !== null)
  const displayName = computed(() => {
    return currentUser.value?.nickname || currentUser.value?.username || '未登录'
  })
  const avatarText = computed(() => {
    const source = displayName.value.trim() || 'RA'
    return source.slice(0, 2).toUpperCase()
  })

  async function login(payload: LoginPayload) {
    const response = await loginRequest(payload)
    applyLoginResponse(response.data)
    await fetchMe({ force: true })
    return response.data
  }

  async function fetchMe(options: { force?: boolean } = {}) {
    if (!hasToken.value) {
      clearAuth()
      throw new Error('missing access token')
    }

    if (!options.force && profileLoaded.value && currentUser.value) {
      return {
        user: currentUser.value,
        roles: roles.value,
        permissions: permissions.value,
      } satisfies CurrentUserResponseData
    }

    if (!options.force && profileRequest.value) {
      return profileRequest.value
    }

    // A failed /me means the local auth context is no longer trustworthy, so
    // we reset everything here instead of letting stale user data linger.
    const request = fetchCurrentUser()
      .then((response) => {
        currentUser.value = response.data.user
        roles.value = response.data.roles
        permissions.value = response.data.permissions
        profileLoaded.value = true
        return response.data
      })
      .catch((error) => {
        clearAuth()
        throw error
      })
      .finally(() => {
        profileRequest.value = null
      })

    profileRequest.value = request
    return request
  }

  async function bootstrap() {
    if (!hasToken.value) {
      return
    }

    await fetchMe()
  }

  async function logout() {
    try {
      if (hasToken.value) {
        await logoutRequest()
      }
    }
    finally {
      clearAuth()
    }
  }

  function clearAuth() {
    accessToken.value = ''
    currentUser.value = null
    roles.value = []
    permissions.value = []
    profileLoaded.value = false
    profileRequest.value = null
    removeToken()
  }

  function applyLoginResponse(payload: LoginResponseData) {
    accessToken.value = payload.access_token
    currentUser.value = payload.user
    profileLoaded.value = false
    setToken(payload.access_token)
  }

  return {
    accessToken,
    currentUser,
    roles,
    permissions,
    hasToken,
    isAuthenticated,
    displayName,
    avatarText,
    login,
    fetchMe,
    bootstrap,
    logout,
    clearAuth,
  }
})
