import type { VersionedData } from '@0x-jerry/utils'
import { useConfig, type UseConfigOption } from './useConfig'

export interface BasicConfigV1 extends VersionedData {
  version: 1
  proxy: string
  listenClipboard: boolean
}

export type BasicConfigLatest = BasicConfigV1

export function useBasicConfig(option?: UseConfigOption<BasicConfigLatest>) {
  const defaultConfig: BasicConfigLatest = {
    version: 1,
    proxy: '',
    listenClipboard: true,
  }

  return useConfig<BasicConfigV1>('config.json', 'data', defaultConfig, {
    ...option,
    migrations: [
      {
        version: 1,
        upgrade(): BasicConfigV1 {
          return defaultConfig
        },
      },
    ],
  })
}
