import type { EndpointItem } from '../types/endpoint'

export const BuiltinEndpointsConfig: Omit<EndpointItem, 'apiKey'>[] = [
  {
    name: '百炼',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    models: [
      'qwen-turbo',
      'qwen-turbo-latest',
      'qwen-plus',
      'qwen-plus-latest',
    ],
  },
]
