<script lang="ts" setup>
import { Optional } from '@0x-jerry/utils'
import { useAsyncData } from '@0x-jerry/vue-kit'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useLocalStorage } from '@vueuse/core'
import { nextTick, onMounted, reactive, ref, useTemplateRef } from 'vue'
import ChatWithHistory from '../components/ChatWithHistory.vue'
import { chatHistoryTable, IChatHistoryModel } from '../database/chatHistory'
import {
  chatHistoryMsgTable,
  IChatHistoryMsgItem,
  IChatHistoryMsgModel,
} from '../database/chatHistoryMsg'
import { promptConfigTable } from '../database/promptConfig'
import { WindowEventName } from '../logic/events'
import { mustache } from '../utils'
import ChatPageHead from './components/ChatPageHead.vue'

const state = reactive({
  messages: [] as IChatHistoryMsgItem[],
  responseMsg: null as Optional<IChatHistoryMsgModel>,
})

const pinned = useLocalStorage('chat-pinned', false)

const payload = useLocalStorage('chat-payload', {
  promptId: null as number | null,
  selectedText: '',
})

const chatRef = useTemplateRef('chatRef')

const chatHistory = ref<IChatHistoryModel>()

const promptConfigApi = useAsyncData(promptConfigTable.getById)

const win = getCurrentWindow()

win.listen(WindowEventName.ChatShow, async (evt) => {
  payload.value = {
    promptId: Number(evt.payload.prompt_id),
    selectedText: evt.payload.selected_text,
  }

  await fetchInitializedData()
})

win.listen(WindowEventName.ChatHide, async () => {
  if (pinned.value) {
    return
  }

  await win.hide()

  chatRef.value?.abortStream()
})

onMounted(async () => {
  await fetchInitializedData()
})

async function fetchInitializedData() {
  if (!payload.value.promptId) {
    return
  }

  await promptConfigApi.load(payload.value.promptId)

  await createChatHistory()
}

async function createChatHistory() {
  const promptTpl = promptConfigApi.data.value?.prompt

  if (!promptTpl) {
    throw new Error(`Prompt config is null`)
  }

  state.messages = []

  const msg = mustache(promptTpl, {
    selection: payload.value.selectedText,
  })

  const msgItem = await chatHistoryMsgTable.getByContent(msg)

  // Restore chat history
  if (msgItem?.chatHistoryId) {
    chatHistory.value = await chatHistoryTable.getById(msgItem.chatHistoryId)

    const msgs = await chatHistoryMsgTable.getMsgs(msgItem.chatHistoryId)

    state.messages = msgs
  } else {
    // todo, generate history name
    const name = promptConfigApi.data.value?.name || 'unknown-prompt-config'

    chatHistory.value = await chatHistoryTable.createOne({
      name,
    })

    await nextTick()
    await chatRef.value?.sendMsg(msg)
  }
}
</script>

<template>
  <div class="page flex flex-col w-400px h-600px bg-white">
    <ChatPageHead :icon="promptConfigApi.data.value?.icon" :title="promptConfigApi.data.value?.name" v-model:pinned="pinned" />

    <template v-if="chatHistory">
      <div class="flex-1 h-0">
        <ChatWithHistory  ref="chatRef" :history-id="chatHistory.id" :prompt-config-id="promptConfigApi.data.value?.id" />
      </div>
    </template>
  </div>
</template>

<style lang="scss" scoped></style>
