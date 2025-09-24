import type { VersionedData } from '@0x-jerry/utils'
import { useConfig, type UseConfigOption } from './useConfig'

export interface BasicConfigV1 extends VersionedData {
  version: 1
  proxy: string
  listenClipboard: boolean
}

export interface BasicConfigV2 extends VersionedData {
  version: 2
  proxy: string
  listenClipboard: boolean
  enabled: boolean
}

export type BasicConfigLatest = BasicConfigV2

export function useBasicConfig(option?: UseConfigOption<BasicConfigLatest>) {
  const defaultConfig: BasicConfigLatest = {
    version: 2,
    proxy: '',
    listenClipboard: true,
    enabled: true,
  }

  return useConfig<BasicConfigLatest>('config.json', 'data', defaultConfig, {
    ...option,
    migrations: [
      {
        version: 1,
        upgrade(): BasicConfigV1 {
          return {
            version: 1,
            proxy: '',
            listenClipboard: true,
          }
        },
      },
      {
        version: 2,
        upgrade(data: BasicConfigV1): BasicConfigV2 {
          return {
            ...data,
            version: 2,
            enabled: true,
          }
        },
      },
    ],
  })
}
