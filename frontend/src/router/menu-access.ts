import type { AppMenuItem } from '@/types/menu'

type RouteAccessOptions = {
  path: string
  menuTree: AppMenuItem[]
  permissions?: string[]
  isSuperAdmin?: boolean
  requiredPermission?: string
}

export type AppTopNavKey =
  | 'workspace'
  | 'permission'
  | 'organization'
  | 'audit'
  | 'settings'
  | 'profile'

export type AppTopNavItem = {
  key: AppTopNavKey
  label: string
  path: string
}

export const LEGACY_MENU_ROUTE_MAP: Record<string, string> = {
  '/placeholder/users': '/system/user',
  '/placeholder/roles': '/system/role',
  '/placeholder/menus': '/system/menu',
  '/placeholder/departments': '/system/dept',
  '/placeholder/posts': '/system/post',
  '/placeholder/dicts': '/system/dict',
  '/placeholder/configs': '/system/config',
  '/placeholder/login-logs': '/audit/login-log',
  '/placeholder/audit-logs': '/audit/operation-log',
  '/placeholder/operation-log': '/audit/operation-log',
}

const ALWAYS_ACCESSIBLE_ROUTE_PATHS = new Set([
  '/dashboard',
  '/system',
  '/profile',
])

const TOP_NAV_BLUEPRINT: Array<{ key: AppTopNavKey, label: string, candidates: string[] }> = [
  { key: 'workspace', label: '工作台', candidates: ['/dashboard'] },
  { key: 'permission', label: '权限中心', candidates: ['/system/role', '/system/menu', '/system/dict'] },
  { key: 'organization', label: '组织架构', candidates: ['/system/user', '/system/dept', '/system/post'] },
  { key: 'audit', label: '审计中心', candidates: ['/audit/login-log', '/audit/operation-log'] },
  { key: 'settings', label: '系统配置', candidates: ['/system/config'] },
  { key: 'profile', label: '个人中心', candidates: ['/profile'] },
]

export function normalizeMenuTreePaths(items: AppMenuItem[]): AppMenuItem[] {
  return items.map((item) => ({
    ...item,
    path: normalizeMenuPath(item.path),
    children: item.children ? normalizeMenuTreePaths(item.children) : item.children,
  }))
}

export function normalizeMenuPath(path?: string) {
  if (!path) {
    return undefined
  }

  return LEGACY_MENU_ROUTE_MAP[path] ?? path
}

export function collectAccessibleMenuPaths(menuTree: AppMenuItem[]) {
  const paths = new Set<string>()

  for (const item of menuTree) {
    collectPathsFromItem(item, paths)
  }

  return paths
}

export function resolveFirstAccessiblePath(menuTree: AppMenuItem[]) {
  for (const item of menuTree) {
    const path = findFirstMenuPath(item)
    if (path) {
      return path
    }
  }

  return '/dashboard'
}

export function canAccessProtectedRoute(options: RouteAccessOptions) {
  const normalizedPath = normalizeRuntimeRoutePath(options.path)

  if (ALWAYS_ACCESSIBLE_ROUTE_PATHS.has(normalizedPath)) {
    return true
  }

  if (options.isSuperAdmin) {
    return true
  }

  const requiredPermission = options.requiredPermission ?? resolveDynamicRoutePermission(normalizedPath)
  if (requiredPermission) {
    return options.permissions?.includes(requiredPermission) ?? false
  }

  return collectAccessibleMenuPaths(options.menuTree).has(normalizedPath)
}

export function resolveTopNavItems(menuTree: AppMenuItem[]) {
  const accessiblePaths = collectAccessibleMenuPaths(menuTree)

  return TOP_NAV_BLUEPRINT.flatMap((item): AppTopNavItem[] => {
    const path = item.candidates.find(candidate =>
      ALWAYS_ACCESSIBLE_ROUTE_PATHS.has(candidate) || accessiblePaths.has(candidate),
    )

    return path
      ? [{
          key: item.key,
          label: item.label,
          path,
        }]
      : []
  })
}

export function resolveActiveTopNavKey(path: string): AppTopNavKey {
  const normalizedPath = normalizeRuntimeRoutePath(path)

  if (normalizedPath.startsWith('/system/role') || normalizedPath.startsWith('/system/menu') || normalizedPath.startsWith('/system/dict')) {
    return 'permission'
  }

  if (normalizedPath.startsWith('/system/user') || normalizedPath.startsWith('/system/dept') || normalizedPath.startsWith('/system/post')) {
    return 'organization'
  }

  if (normalizedPath.startsWith('/audit/')) {
    return 'audit'
  }

  if (normalizedPath.startsWith('/system/config') || normalizedPath === '/system') {
    return 'settings'
  }

  if (normalizedPath.startsWith('/profile')) {
    return 'profile'
  }

  return 'workspace'
}

export function isRouteActive(currentPath: string, itemPath?: string) {
  const normalizedCurrentPath = normalizeRuntimeRoutePath(currentPath)
  const normalizedItemPath = normalizeMenuPath(itemPath)

  if (!normalizedItemPath) {
    return false
  }

  if (normalizedCurrentPath === normalizedItemPath) {
    return true
  }

  return normalizedCurrentPath.startsWith(`${normalizedItemPath}/`)
}

function collectPathsFromItem(item: AppMenuItem, paths: Set<string>) {
  const normalizedPath = normalizeMenuPath(item.path)
  if (normalizedPath) {
    paths.add(normalizedPath)
  }

  item.children?.forEach(child => collectPathsFromItem(child, paths))
}

function findFirstMenuPath(item: AppMenuItem): string | null {
  const normalizedPath = normalizeMenuPath(item.path)
  if (normalizedPath) {
    return normalizedPath
  }

  for (const child of item.children ?? []) {
    const path = findFirstMenuPath(child)
    if (path) {
      return path
    }
  }

  return null
}

function normalizeRuntimeRoutePath(path: string) {
  return normalizeMenuPath(path) ?? path
}

function resolveDynamicRoutePermission(path: string) {
  if (path.startsWith('/system/role/') && path.endsWith('/permission')) {
    return 'system:role:assign-permission'
  }

  return undefined
}
