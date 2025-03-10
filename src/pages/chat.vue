<script lang="ts" setup>
import { watchImmediate } from '@vueuse/core'
import { computed, reactive } from 'vue'
import type { ChatHistory } from '../components/Chat/types'
import ChatMessages from '../components/Chat/ChatMessages.vue'
import { nanoid } from '@0x-jerry/utils'
import { getPromptConf } from '../logic/config'
import { mustache } from '../utils'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import DraggableArea from '../components/DraggableArea.vue'
import CloseWindow from '../components/CloseWindow.vue'
import { getSelectedText } from '../logic/commands'
import { getCurrentWindow } from '@tauri-apps/api/window'

const urlParams = reactive({
	promptId: '',
})

const win = getCurrentWindow()

win.listen('prompt-id-changed', (evt) => {
	urlParams.promptId = evt.payload as string
})

const promptConf = computed(() =>
	urlParams.promptId ? getPromptConf(urlParams.promptId) : undefined,
)

watchImmediate(
	() => urlParams.promptId,
	async () => {
		if (!urlParams.promptId) return

		state.ready = false
		state.chatHistory.messages = []

		await initMessages()
	},
)

const state = reactive({
	ready: false,
	chatHistory: {
		id: nanoid(),
		name: 'temp',
		messages: [],
	} as ChatHistory,
})

async function initMessages() {
	state.chatHistory.messages.push({
		role: 'user',
		content: mustache(promptConf.value?.prompt || '', {
			selection: (await getSelectedText()) || '',
		}),
	})

	state.ready = true
}
</script>

<template>
  <AutoResizeContainer :width="400">
    <div class="page bg-white">
      <div class="title flex items-center px-2 py-1 border-(0 b solid gray)">
        <DraggableArea class="flex-1">
          This is Draggable
        </DraggableArea>
        <CloseWindow />
      </div>

      <div class="page flex flex-col bg-white min-h-500px">
        <ChatMessages
          v-if="state.ready && promptConf?.id"
          v-model="state.chatHistory"
          :prompt-id="promptConf.id"
        />
      </div>
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
