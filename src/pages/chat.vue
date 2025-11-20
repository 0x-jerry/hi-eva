<script lang="ts" setup>
import { nanoid } from '@0x-jerry/utils'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { computed, nextTick, onMounted, reactive, ref, watch } from 'vue'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import CarbonIcon from '../components/CarbonIcon.vue'
import ChatMessages from '../components/Chat/ChatMessages.vue'
import type { ChatHistory } from '../components/Chat/types'
import CloseWindow from '../components/CloseWindow.vue'
import DraggableArea from '../components/DraggableArea.vue'
import Icon from '../components/Icon.vue'
import { selectionTable } from '../database'
import { commands } from '../logic/commands'
import { getPromptConf } from '../logic/config'
import { mustache } from '../utils'

const chatRef = ref<InstanceType<typeof ChatMessages>>()

const initialChat: ChatHistory = {
  id: nanoid(),
  name: 'temp',
  messages: [],
}

const state = reactive({
  promptId: '',
  selectedText: '',
  pinned: false,
  ready: false,
  chatHistory: initialChat,
})

const promptConf = computed(() =>
  state.promptId ? getPromptConf(state.promptId) : undefined,
)

const win = getCurrentWindow()

win.listen('show-chat', async (evt) => {
  const payload = evt.payload as { prompt_id: string; selected_text: string }
  state.promptId = payload.prompt_id
  state.selectedText = payload.selected_text

  await selectionTable.createOne({
    selected: payload.selected_text,
    promptName: promptConf.value?.name || '',
  })

  await resetChatMessage()
})

win.listen('hide-chat', async () => {
  if (state.pinned) {
    return
  }

  await win.hide()
})

watch(
  () => state.pinned,
  () => commands.setChatPinned({ pinned: state.pinned }),
)

onMounted(async () => {
  await commands.applyAppearance()
})

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
      selection: state.selectedText,
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
            <CarbonIcon v-if="promptConf.icon" :name="promptConf.icon" class="text-xl" />
            <span class="pl-2">
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

<style lang="scss" scoped></style>
