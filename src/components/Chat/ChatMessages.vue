<script setup lang="ts">
import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs'
import Button from 'primevue/button'
import Textarea from 'primevue/textarea'
// biome-ignore lint/style/useImportType: <explanation>
import ContextMenu from 'primevue/contextmenu'
import type { ChatHistory } from './types'
import type { MenuItem } from 'primevue/menuitem'
import { remove } from 'lodash-es'
import { reactive, ref } from 'vue'
import { chatWithPrompt } from './chat'
import Markdown from '../Markdown.vue'

export interface ChatMessagesProps {
	promptId: string
}

const props = defineProps<ChatMessagesProps>()

const chat = defineModel<ChatHistory>()

const emit = defineEmits(['click-title'])

const menuRef = ref<InstanceType<typeof ContextMenu>>()

const contextMenuState = reactive({
	selectMsg: null as ChatCompletionMessageParam | null,
})

let requestAbort: AbortController | null = null

const state = reactive({
	reply: '',
	isThinking: false,
	isReplying: false,
})

function handleKeydown(evt: KeyboardEvent) {
	if (evt.key === 'Enter') {
		onSend()
	}
}

async function onSend() {
	if (!chat.value) return
	if (state.isThinking || state.isReplying) return

	const reply = state.reply.trim()
	if (!reply) return
	state.reply = ''

	chat.value.messages.push({
		role: 'user',
		content: reply,
	})

	await continueChat()
}

async function continueChat() {
	if (!chat.value) return

	state.isThinking = true
	state.isReplying = false

	await chatWith(chat.value.messages.slice())

	state.isThinking = false
	state.isReplying = false
}

async function chatWith(messages: ChatCompletionMessageParam[]) {
	if (!chat.value) {
		return
	}

	chat.value.messages.push({
		role: 'assistant',
		content: '',
	})

	try {
		const respStream = await chatWithPrompt(messages, {
			promptId: props.promptId,
		})

		state.isThinking = false
		state.isReplying = true

		// biome-ignore lint/style/noNonNullAssertion: <explanation>
		const msg = chat.value.messages.at(-1)!

		if (requestAbort && !requestAbort.signal.aborted) {
			requestAbort.abort()
		}

		requestAbort = new AbortController()

		for await (const chunkItem of respStream) {
			const chunkConent = chunkItem.choices.at(0)?.delta.content || ''
			msg.content += chunkConent
		}
	} catch (error) {
		const lastMsg = chat.value.messages.at(-1)

		if (lastMsg) {
			lastMsg.content += String(error)
		}
	}

	requestAbort = null

	state.isReplying = false
}

const menuItems: MenuItem[] = [
	{
		label: '重新生成',
		icon: 'pi pi-sync',
		command() {
			if (!chat.value) return
			if (!contextMenuState.selectMsg) return

			const idx = chat.value.messages.indexOf(contextMenuState.selectMsg)
			if (idx == null) return

			chat.value.messages = chat.value.messages.slice(0, idx)

			continueChat()
		},
	},
	{
		label: '删除',
		icon: 'pi pi-trash',
		command() {
			if (requestAbort && !requestAbort.signal.aborted) {
				requestAbort.abort()
			}

			state.isReplying = false
			state.isThinking = false

			remove(
				chat.value?.messages || [],
				(msg) => msg === contextMenuState.selectMsg,
			)
		},
	},
]

function showContextmenu(event: Event, msg: ChatCompletionMessageParam) {
	contextMenuState.selectMsg = msg
	menuRef.value?.show(event)
}

defineExpose({
	continueChat,
})
</script>

<template>
  <div class="flex flex-col bg-white min-h-100px max-h-600px">
    <div class="chat-msgs overflow-auto p-4">
      <div class="chat-msg-item flex flex-col mb-4" v-for="(msg, _idx) in chat?.messages.toReversed()">
        <div class="role flex text-(2xl)">
          <span v-if="msg.role === 'user'" class="i-carbon-user-avatar-filled-alt text-blue-4"></span>
          <span v-else class="i-carbon-machine-learning text-rose-4"></span>
        </div>
        <div class="chat-msg-content ml-3 mt-2 p-2 bg-light-5 rounded border-(1 solid gray-2)"
          @contextmenu.prevent="showContextmenu($event, msg)">
          <template v-if="msg.content">
            <Markdown :content="msg.content as string" />
          </template>
          <template v-else>
            <i class="pi pi-spin pi-spinner"></i>
            思考中...
          </template>
        </div>
      </div>
    </div>

    <ContextMenu ref="menuRef" :model="menuItems" />

    <div class="reply-bar border-(0 t solid gray-2) px-2 py-2 flex gap-2">
      <Textarea rows="3" class="flex-1" v-model="state.reply" @keydown="handleKeydown" placeholder="Please input" />
      <Button :disabled="state.isReplying || state.isThinking" @click=onSend>Send</Button>
    </div>
  </div>
</template>

<style scoped>
.chat-msgs {
  display: flex;
  flex-direction: column-reverse;
}
</style>