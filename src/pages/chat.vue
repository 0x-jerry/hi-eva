<script lang="ts" setup>
import { Optional } from '@0x-jerry/utils'
import { useAsyncData } from '@0x-jerry/vue-kit'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { nextTick, reactive, ref, useTemplateRef } from 'vue'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
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
  selectedText: '',
  pinned: false,
  messages: [] as IChatHistoryMsgItem[],
  responseMsg: null as Optional<IChatHistoryMsgModel>,
})

const chatRef = useTemplateRef('chatRef')

const chatHistory = ref<IChatHistoryModel>()

const promptConfigApi = useAsyncData(promptConfigTable.getById)

const win = getCurrentWindow()

win.listen(WindowEventName.ChatShow, async (evt) => {
  const payload = evt.payload
  state.selectedText = payload.selected_text

  await promptConfigApi.load(Number(payload.prompt_id))

  await createChatHistory()
})

win.listen(WindowEventName.ChatHide, async () => {
  if (state.pinned) {
    return
  }

  await win.hide()

  chatRef.value?.abortStream()
})

async function createChatHistory() {
  const promptTpl = promptConfigApi.data.value?.prompt

  if (!promptTpl) {
    throw new Error(`Prompt config is null`)
  }

  state.messages = []

  const msg = mustache(promptTpl, {
    selection: state.selectedText,
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
  <AutoResizeContainer :width="400">
    <div class="page bg-white">
      <ChatPageHead :icon="promptConfigApi.data.value?.icon" :title="promptConfigApi.data.value?.name" v-model:pinned="state.pinned" />

      <template v-if="chatHistory">
        <ChatWithHistory ref="chatRef" :history-id="chatHistory.id" :prompt-config-id="promptConfigApi.data.value?.id" />
      </template>
    </div>
  </AutoResizeContainer>
</template>

<style lang="scss" scoped></style>
