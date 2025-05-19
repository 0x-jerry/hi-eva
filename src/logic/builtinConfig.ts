export interface BuiltinConfigItem  {
  name: string
  baseUrl: string
  models: string[]
}

export const BuiltinEndpointsConfig: BuiltinConfigItem[] = [
  {
    name: '百炼',
    baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
    models: ['qwen-turbo'],
  },
]
