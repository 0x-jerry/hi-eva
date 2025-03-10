<script lang="ts" setup>
import { useUrlSearchParams } from '@vueuse/core'
import { reactive } from 'vue'
import type { ChatHistory } from '../components/Chat/types'
import ChatMessages from '../components/Chat/ChatMessages.vue'
import { nanoid } from '@0x-jerry/utils'
import { getPromptConf } from '../logic/config'
import { mustache } from '../utils'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import DraggableArea from '../components/DraggableArea.vue'
import CloseWindow from '../components/CloseWindow.vue'
import { getSelectedText } from '../logic/commands'

interface RouteParams {
	readonly promptId: string
}

const urlParams = useUrlSearchParams<RouteParams>('hash')

const promptConf = getPromptConf(urlParams.promptId)

const state = reactive({
	ready: false,
	chatHistory: {
		id: nanoid(),
		name: 'temp',
		messages: [],
	} as ChatHistory,
})

initMessages()

async function initMessages() {
	state.chatHistory.messages.push({
		role: 'user',
		content: mustache(promptConf?.prompt || '', {
			selection: (await getSelectedText()) || '',
		}),
	})

	state.ready = true
}
</script>

<template>
  <AutoResizeContainer>
    <div class="page bg-white">
      <DraggableArea class="px-2 py-1 border-(0 b solid gray) flex"> 
        <span class="flex-1">
        This is Draggable 
        </span>
        <div>
          <CloseWindow />
        </div>

      </DraggableArea>
      <div class="page flex flex-col bg-white min-h-600px">
        <ChatMessages
          v-if="state.ready"
          v-model="state.chatHistory"
          :prompt-id="urlParams.promptId"
        />
      </div>
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
