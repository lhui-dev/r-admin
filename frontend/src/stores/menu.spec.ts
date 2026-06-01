import { beforeEach, describe, expect, it, vi } from 'vitest'

import { pinia } from '@/stores'
import { useMenuStore } from '@/stores/menu'

vi.mock('@/services/menu', () => ({
  loadCurrentMenus: vi.fn(async () => [
    {
      id: 'workspace',
      name: 'workspace',
      title: '工作台',
      children: [
        {
          id: 'dashboard',
          name: 'dashboard',
          title: '概览看板',
          path: '/dashboard',
        },
      ],
    },
    {
      id: 'access',
      name: 'access',
      title: '权限管理',
      children: [
        {
          id: 'users',
          name: 'users',
          title: '用户管理',
          permission: 'system:user:list',
        },
        {
          id: 'roles',
          name: 'roles',
          title: '角色管理',
          permission: 'system:role:list',
        },
        {
          id: 'permissions',
          name: 'permissions',
          title: '权限点管理',
          permission: 'system:permission:list',
        },
        {
          id: 'menus',
          name: 'menus',
          title: '菜单管理',
          permission: 'system:menu:list',
        },
      ],
    },
    {
      id: 'system',
      name: 'system',
      title: '系统与个人',
      children: [
        {
          id: 'profile',
          name: 'profile',
          title: '个人信息',
          path: '/profile',
        },
      ],
    },
  ]),
}))

describe('menu store', () => {
  beforeEach(() => {
    useMenuStore(pinia).resetMenus()
  })

  it('initializes visible menus from the shared menu loader', async () => {
    const menuStore = useMenuStore(pinia)

    await menuStore.initMenus({
      permissions: ['system:user:list', 'system:menu:list'],
      isSuperAdmin: false,
    })

    expect(menuStore.initialized).toBe(true)
    expect(menuStore.visibleMenuTree).toHaveLength(3)
    expect(menuStore.visibleMenuTree[1].children?.map((item) => item.id)).toEqual(['users', 'menus'])
  })

  it('keeps all permission-bound menus visible for super admins', async () => {
    const menuStore = useMenuStore(pinia)

    await menuStore.initMenus({
      permissions: [],
      isSuperAdmin: true,
    })

    expect(menuStore.visibleMenuTree[1].children?.map((item) => item.id)).toEqual([
      'users',
      'roles',
      'permissions',
      'menus',
    ])
  })

  it('resets menu state when requested', async () => {
    const menuStore = useMenuStore(pinia)

    await menuStore.initMenus()
    menuStore.resetMenus()

    expect(menuStore.initialized).toBe(false)
    expect(menuStore.menuTree).toEqual([])
  })
})
