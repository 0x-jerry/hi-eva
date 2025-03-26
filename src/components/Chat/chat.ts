import OpenAI from 'openai'
import { getEndpointConf, getPromptConf } from '../../logic/config'

export interface ChatWithOption {
  promptId: string
}

export function chatWithPrompt(
  messages: OpenAI.Chat.ChatCompletionMessageParam[],
  opt: ChatWithOption,
) {
  const conf = getPromptConf(opt.promptId)

  if (!conf?.endpointId) {
    throw new Error('Can not find the prompt setting')
  }

  const endpointConf = getEndpointConf(conf.endpointId)

  if (!endpointConf) {
    throw new Error('Pease select an endpoint')
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
