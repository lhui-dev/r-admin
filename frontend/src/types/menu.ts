export type AppMenuItem = {
  id: string
  name: string
  title: string
  path?: string
  icon?: string
  permission?: string
  hidden?: boolean
  description?: string
  children?: AppMenuItem[]
}
