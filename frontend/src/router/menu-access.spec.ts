import { describe, expect, it } from 'vitest'

import {
  isRouteActive,
  normalizeMenuPath,
  resolveActiveTopNavKey,
  resolveTopNavItems,
} from '@/router/menu-access'

describe('menu access helpers', () => {
  it('normalizes legacy placeholder paths to real routes', () => {
    expect(normalizeMenuPath('/placeholder/roles')).toBe('/system/role')
  })

  it('resolves top navigation targets from accessible menu paths', () => {
    const items = resolveTopNavItems([
      {
        id: 'system',
        name: 'system',
        title: '系统管理',
        children: [
          {
            id: 'users',
            name: 'users',
            title: '用户管理',
            path: '/system/user',
          },
          {
            id: 'roles',
            name: 'roles',
            title: '角色管理',
            path: '/system/role',
          },
        ],
      },
      {
        id: 'audit',
        name: 'audit',
        title: '日志审计',
        children: [
          {
            id: 'audit-login',
            name: 'audit-login',
            title: '登录日志',
            path: '/audit/login-log',
          },
        ],
      },
    ])

    expect(items.map(item => item.key)).toEqual([
      'workspace',
      'permission',
      'organization',
      'audit',
      'profile',
    ])
    expect(items.find(item => item.key === 'permission')?.path).toBe('/system/role')
    expect(items.find(item => item.key === 'organization')?.path).toBe('/system/user')
  })

  it('resolves the active top navigation key from routed pages', () => {
    expect(resolveActiveTopNavKey('/system/role/310/permission')).toBe('permission')
    expect(resolveActiveTopNavKey('/system/dept')).toBe('organization')
    expect(resolveActiveTopNavKey('/audit/operation-log')).toBe('audit')
    expect(resolveActiveTopNavKey('/system/config')).toBe('settings')
  })

  it('treats nested routes as active for their parent menu item', () => {
    expect(isRouteActive('/system/role/310/permission', '/system/role')).toBe(true)
    expect(isRouteActive('/system/user', '/system/role')).toBe(false)
  })
})
