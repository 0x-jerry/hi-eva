import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs'

export type ChatMessageWithDb = ChatCompletionMessageParam & { __dbId?: number }

export interface ChatHistory {
  id: string
  name: string
  messages: ChatMessageWithDb[]
}
