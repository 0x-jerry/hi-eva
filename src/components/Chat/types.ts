import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs'

export interface ChatHistory {
	id: string
	name: string
	messages: ChatCompletionMessageParam[]
}
