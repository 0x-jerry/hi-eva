export interface AppBasicConfig {
  version: number
  proxy: string

  autostart: boolean

  enableAutoTrigger: boolean
  enableListenClipboard: boolean

  enableGlobalShortcut: boolean
  globalShortcut: string
}
