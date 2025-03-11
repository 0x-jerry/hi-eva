import OpenAI from 'openai'
import { aiEndPointConfigs, promptConfigs } from '../../logic/config'

export interface ChatWithOption {
	promptId: string
}

export function chatWithPrompt(
	messages: OpenAI.Chat.ChatCompletionMessageParam[],
	opt: ChatWithOption,
) {
	const conf = promptConfigs.value.find((n) => n.id === opt.promptId)

	if (!conf) {
		throw new Error('Can not find the prompt setting')
	}

	const endpointConf = aiEndPointConfigs.value.find(
		(n) => n.id === conf.enpointId,
	)

	if (!endpointConf) {
		throw new Error('Pease select an enpoint')
	}

	if (!conf.model) {
		throw new Error('Pease select an model')
	}

	const client = new OpenAI({
		baseURL: endpointConf.baseUrl,
		apiKey: endpointConf.apiKey,
		dangerouslyAllowBrowser: true,
	})

	const result = client.chat.completions.create({
		model: conf.model,
		messages,
		stream: true,
	})

	return result
}
