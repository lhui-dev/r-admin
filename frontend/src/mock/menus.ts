  import type { AppMenuItem } from '@/types/menu'

export function buildMockMenuTree(): AppMenuItem[] {
  return [
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
          icon: 'histogram',
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
          path: '/placeholder/users',
          icon: 'user',
          permission: 'system:user:list',
          description: '用户管理页建设中。',
        },
        {
          id: 'roles',
          name: 'roles',
          title: '角色管理',
          path: '/placeholder/roles',
          icon: 'collection-tag',
          permission: 'system:role:list',
          description: '角色管理页建设中。',
        },
        {
          id: 'permissions',
          name: 'permissions',
          title: '权限点管理',
          path: '/placeholder/permissions',
          icon: 'compass',
          permission: 'system:permission:list',
          description: '权限点管理页建设中。',
        },
        {
          id: 'menus',
          name: 'menus',
          title: '菜单管理',
          path: '/placeholder/menus',
          icon: 'document',
          permission: 'system:menu:list',
          description: '菜单管理页建设中。',
        },
      ],
    },
    {
      id: 'org-audit',
      name: 'org-audit',
      title: '组织与审计',
      children: [
        {
          id: 'departments',
          name: 'departments',
          title: '部门管理',
          path: '/placeholder/departments',
          icon: 'credit-card',
          permission: 'system:dept:list',
          description: '部门管理页建设中。',
        },
        {
          id: 'audit-logs',
          name: 'audit-logs',
          title: '审计日志',
          path: '/placeholder/audit-logs',
          icon: 'wallet',
          permission: 'system:log:operation:list',
          description: '审计日志页建设中。',
        },
        {
          id: 'system',
          name: 'system',
          title: '系统设置',
          path: '/system',
          icon: 'setting',
        },
      ],
    },
    {
      id: 'profile-center',
      name: 'profile-center',
      title: '个人中心',
      children: [
        {
          id: 'profile',
          name: 'profile',
          title: '个人信息',
          path: '/profile',
          icon: 'user',
        },
      ],
    },
  ]
}
