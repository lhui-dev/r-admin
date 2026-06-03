import { fetchCurrentMenus } from '@/api/menu'
import { normalizeMenuTreePaths } from '@/router/menu-access'
import { buildMockMenuTree } from '@/mock/menus'
import type { AppMenuItem } from '@/types/menu'

export type LoadCurrentMenusOptions = {
  permissions?: string[]
  isSuperAdmin?: boolean
}

export async function loadCurrentMenus(_options: LoadCurrentMenusOptions = {}): Promise<AppMenuItem[]> {
  try {
    const response = await fetchCurrentMenus()
    return normalizeMenuTreePaths(response.data.menus)
  }
  catch (error) {
    // The menu endpoint is still being phased in. Falling back here keeps the
    // current UI usable while the backend contract catches up.
    if (shouldFallbackToMockMenus(error)) {
      return normalizeMenuTreePaths(buildMockMenuTree())
    }

    throw error
  }
}

function shouldFallbackToMockMenus(error: unknown) {
  if (typeof error === 'string') {
    return /not found|not implemented|cannot get|404/i.test(error)
  }

  if (typeof error === 'object' && error !== null) {
    const record = error as Record<string, unknown>
    const rawCode = String(record.code ?? '')
    const code = Number(record.code ?? 0)
    const message = String(record.message ?? '')
    const responseStatus = Number(
      (record.response as { status?: number } | undefined)?.status ?? 0,
    )

    return code === 404
      || responseStatus === 404
      || rawCode === 'ERR_NETWORK'
      || /not found|not implemented|network error/i.test(message)
  }

  return false
}
