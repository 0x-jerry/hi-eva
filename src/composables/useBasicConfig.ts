import type { VersionedData } from '@0x-jerry/utils'
import { type UseConfigOption, useConfig } from './useConfig'

export interface BasicConfig extends VersionedData {
  proxy: string
  enableAutoTrigger: boolean
  enableListenClipboard: boolean

  enableGlobalShortcut: boolean
  globalShortcut: string
}

export function useBasicConfig(option?: UseConfigOption<BasicConfig>) {
  const defaultConfig: BasicConfig = {
    version: 3,
    proxy: '',
    enableAutoTrigger: true,
    enableListenClipboard: true,

    enableGlobalShortcut: true,
    globalShortcut: '',
  }

  return useConfig<BasicConfig>('config.json', 'data', defaultConfig, {
    ...option,
  })
}
