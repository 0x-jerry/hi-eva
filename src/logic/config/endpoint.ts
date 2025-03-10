import { useLocalStorage } from '@vueuse/core'

export interface AIEndpointConfig {
	id: string
	builtin?: boolean
	label: string
	baseUrl: string
	apiKey: string
	models: string[]
}

const builtinEndpointConfigs: AIEndpointConfig[] = [
	{
		id: 'dashscope',
		builtin: true,
		label: '百炼',
		baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
		apiKey: '',
		models: [],
	},
]

export const aiEndPointConfigs = useLocalStorage<AIEndpointConfig[]>(
	'ai-endpoint-config',
	builtinEndpointConfigs,
)
