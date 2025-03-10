<script lang='ts' setup>
import { useUrlSearchParams } from '@vueuse/core'
import { reactive } from 'vue'
import type { ChatHistory } from '../components/Chat/types'
import ChatMessages from '../components/Chat/ChatMessages.vue'
import { nanoid } from '@0x-jerry/utils'
import type OpenAI from 'openai'
import { getPromptConf } from '../logic/config'
import { mustache } from '../utils'

interface RouteParams {
	readonly toolbarPromptId: string
	readonly selectedText: string
}

const urlParams = useUrlSearchParams<RouteParams>('hash')

const promptConf = getPromptConf(urlParams.toolbarPromptId)

const messages: OpenAI.Chat.Completions.ChatCompletionMessageParam[] = [
	{
		role: 'user',
		content: mustache(promptConf?.prompt || '', {
			selection: urlParams.selectedText,
		}),
	},
]

const state = reactive({
	messages: {
		id: nanoid(),
		name: 'temp',
		messages,
	} as ChatHistory,
})
</script>

<template>
  <div class="page">
    <ChatMessages v-model="state.messages" :prompt-id="urlParams.toolbarPromptId" />
  </div>
</template>

<style lang='less' scoped></style>