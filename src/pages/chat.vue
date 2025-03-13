<script lang="ts" setup>
import { nanoid } from '@0x-jerry/utils'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { computed, nextTick, reactive, ref } from 'vue'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import ChatMessages from '../components/Chat/ChatMessages.vue'
import type { ChatHistory } from '../components/Chat/types'
import CloseWindow from '../components/CloseWindow.vue'
import DraggableArea from '../components/DraggableArea.vue'
import Icon from '../components/Icon.vue'
import { getPromptConf } from '../logic/config'
import { mustache } from '../utils'
import { commands } from '../logic/commands'

const state = reactive({
  promptId: '',
  pinned: false,
  ready: false,
  chatHistory: {
    id: nanoid(),
    name: 'temp',
    messages: [],
  } as ChatHistory,
})

const chatRef = ref<InstanceType<typeof ChatMessages>>()

const win = getCurrentWindow()

win.listen('show-chat', async (evt) => {
  state.promptId = evt.payload as string
  await resetChatMessage()
})

const promptConf = computed(() =>
  state.promptId ? getPromptConf(state.promptId) : undefined,
)

async function resetChatMessage() {
  if (!state.promptId) return

  state.ready = false
  state.chatHistory.messages = []

  await initMessages()
}

async function initMessages() {
  state.chatHistory.messages.push({
    role: 'user',
    content: mustache(promptConf.value?.prompt || '', {
      selection: (await commands.getSelectedText()) || '',
    }),
  })

  state.ready = true
  await nextTick()

  chatRef.value?.continueChat()
}

async function togglePinWindow() {
  state.pinned = !state.pinned
  await win.setAlwaysOnTop(state.pinned)
}
</script>

<template>
  <AutoResizeContainer :width="400">
    <div class="page bg-white">
      <div class="title flex border-(0 b solid gray) h-8 select-none">
        <div class="flex items-center pl-1 cursor-pointer" @click="togglePinWindow">
          <Icon v-if="state.pinned" class="i-carbon:pin-filled" />
          <Icon v-else class="i-carbon:pin" />
        </div>
        <div class="border-(0 r solid gray) ml-1 mr-2">&nbsp;</div>
        <DraggableArea class="flex-1 flex items-center gap-1">
          <template v-if="promptConf">
            <Icon v-if="promptConf.icon" :class="promptConf.icon" />
            <span class="">
              {{ promptConf?.name }}
            </span>
          </template>
        </DraggableArea>
        <div class="icons h-full">
          <CloseWindow class="h-full" />
        </div>
      </div>

      <ChatMessages ref="chatRef" v-if="state.ready && promptConf?.id" v-model="state.chatHistory"
        :prompt-id="promptConf.id" />
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
