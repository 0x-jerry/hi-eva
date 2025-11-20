import type { VersionedData } from '@0x-jerry/utils'
import { type UseConfigOption, useConfig } from './useConfig'

export interface EndpointConfig {
  id: string
  builtin?: boolean
  label: string
  baseUrl: string
  apiKey: string
  models: string[]
}

export interface EndpointsConfigV1 extends VersionedData {
  version: 1
  items: EndpointConfig[]
}

export type EndpointsConfigLatest = EndpointsConfigV1

const builtinEndpointConfigs: EndpointConfig[] = [
  {
    id: 'DashScope',
    builtin: true,
    label: '百炼',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    apiKey: '',
    models: [],
  },
]

export function useEndpointsConfig(
  option?: UseConfigOption<EndpointsConfigLatest>,
) {
  const defaultConfig: EndpointsConfigLatest = {
    version: 1,
    items: builtinEndpointConfigs,
  }

  return useConfig<EndpointsConfigV1>('endpoints.json', 'data', defaultConfig, {
    ...option,
    migrations: [
      {
        version: 1,
        upgrade(): EndpointsConfigV1 {
          return defaultConfig
        },
      },
    ],
  })
}
