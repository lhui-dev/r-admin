import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createMemoryHistory } from 'vue-router'

import { createAppRouter } from '@/router'
import { pinia } from '@/stores'
import { useAuthStore } from '@/stores/auth'
import { useMenuStore } from '@/stores/menu'

const mockUser = {
  id: 1000,
  username: 'admin',
  nickname: '超级管理员',
  real_name: '平台管理员',
  is_super_admin: true,
}

describe('auth router guards', () => {
  beforeEach(() => {
    window.localStorage.clear()
    vi.restoreAllMocks()
    useAuthStore(pinia).clearAuth()
    useMenuStore(pinia).resetMenus()
  })

  it('redirects unauthenticated access to login with redirect query', async () => {
    const router = createAppRouter(createMemoryHistory())

    await router.push('/dashboard')

    expect(router.currentRoute.value.path).toBe('/login')
    expect(router.currentRoute.value.query.redirect).toBe('/dashboard')
  })

  it('allows protected navigation after bootstrap restores the user profile', async () => {
    const authStore = useAuthStore(pinia)
    const menuStore = useMenuStore(pinia)
    authStore.accessToken = 'mock-token'
    authStore.currentUser = null

    const bootstrapSpy = vi.spyOn(authStore, 'bootstrap').mockImplementation(async () => {
      authStore.currentUser = mockUser
    })

    const router = createAppRouter(createMemoryHistory())
    await router.push('/dashboard')

    expect(bootstrapSpy).toHaveBeenCalledTimes(1)
    expect(router.currentRoute.value.path).toBe('/dashboard')
    expect(menuStore.initialized).toBe(true)
  })

  it('returns to login when bootstrap cannot recover the current user', async () => {
    const authStore = useAuthStore(pinia)
    authStore.accessToken = 'expired-token'
    authStore.currentUser = null

    const bootstrapSpy = vi.spyOn(authStore, 'bootstrap').mockRejectedValue(new Error('expired'))

    const router = createAppRouter(createMemoryHistory())
    await router.push('/profile')

    expect(bootstrapSpy).toHaveBeenCalled()
    expect(router.currentRoute.value.path).toBe('/login')
    expect(router.currentRoute.value.query.redirect).toBe('/profile')
  })

  it('redirects authenticated users away from the login page', async () => {
    const authStore = useAuthStore(pinia)
    authStore.accessToken = 'active-token'
    authStore.currentUser = mockUser

    const router = createAppRouter(createMemoryHistory())
    await router.push('/profile')
    await router.push('/login')

    expect(router.currentRoute.value.path).toBe('/dashboard')
  })
})
