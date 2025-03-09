import { useLocalStorage } from '@vueuse/core'
import OpenAI from 'openai'

export function chatWith(messages: OpenAI.Chat.ChatCompletionMessageParam[]) {
	const setting = useLocalStorage('ai-config', [])

	const client = new OpenAI({
		baseURL: '',
		apiKey: '',
	})
}
