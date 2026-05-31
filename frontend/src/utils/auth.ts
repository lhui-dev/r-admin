const ACCESS_TOKEN_KEY = 'r-admin-access-token'

export function getToken() {
  return window.localStorage.getItem(ACCESS_TOKEN_KEY) ?? ''
}

export function setToken(token: string) {
  window.localStorage.setItem(ACCESS_TOKEN_KEY, token)
}

export function removeToken() {
  window.localStorage.removeItem(ACCESS_TOKEN_KEY)
}
