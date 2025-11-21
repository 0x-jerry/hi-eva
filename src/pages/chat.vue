<script lang="ts" setup>
import { createPromise, isArray, Optional } from '@0x-jerry/utils'
import { useAsyncData, useLoading } from '@0x-jerry/vue-kit'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { debounce } from 'lodash-es'
import OpenAI from 'openai'
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream.mjs'
import { ChatCompletionMessageParam } from 'openai/resources/index.mjs'
import { onMounted, reactive, ref } from 'vue'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import ChatRoom from '../components/Chat/ChatRoom.vue'
import { selectionTable } from '../database'
import { chatHistoryTable, IChatHistoryModel } from '../database/chatHistory'
import {
  chatHistoryMsgTable,
  IChatHistoryMsgItem,
  IChatHistoryMsgModel,
} from '../database/chatHistoryMsg'
import { promptConfigTable } from '../database/promptConfig'
import { ChatRole } from '../logic/chat'
import { commands } from '../logic/commands'
import { WindowEventName } from '../logic/events'
import { mustache } from '../utils'
import ChatPageHead from './components/ChatPageHead.vue'

const state = reactive({
  promptId: '',
  selectedText: '',
  pinned: false,
  ready: false,
  messages: [] as IChatHistoryMsgItem[],
  responseMsg: null as Optional<IChatHistoryMsgModel>,
})

let streamRef: Optional<ChatCompletionStream>

const chatHistory = ref<IChatHistoryModel>()

const promptConfigApi = useAsyncData(promptConfigTable.getById)

const win = getCurrentWindow()

win.listen(WindowEventName.ShowChat, async (evt) => {
  const payload = evt.payload
  state.promptId = payload.prompt_id
  state.selectedText = payload.selected_text

  await promptConfigApi.load(Number(payload.prompt_id))

  await selectionTable.createOne({
    selected: payload.selected_text,
    promptName: promptConfigApi.data.value?.name || 'unknown',
  })

  await createChatHistory()
})

win.listen(WindowEventName.HideChat, async () => {
  if (state.pinned) {
    return
  }

  await win.hide()
})

onMounted(async () => {
  await commands.applyAppearance()
})

async function createChatHistory() {
  if (!state.promptId) return

  state.ready = false

  // todo, generate history name
  const name = promptConfigApi.data.value?.name || 'unknown-prompt-config'

  chatHistory.value = await chatHistoryTable.createOne({
    name,
  })

  await initMessages()
}

async function initMessages() {
  const promptTpl = promptConfigApi.data.value?.prompt

  if (!promptTpl) {
    throw new Error(`Prompt config is null`)
  }

  state.messages = []

  const msg = mustache(promptTpl, {
    selection: state.selectedText,
  })

  await handleSendMsg(msg)
}

async function updateChatTitle(newTitle: string) {
  const history = chatHistory.value
  if (!history) {
    throw new Error(`Chat history is not initialized`)
  }

  await chatHistoryTable.updateOne({
    ...history,
    name: newTitle,
  })

  chatHistory.value = await chatHistoryTable.getById(history.id)
}

const handleSendMsg = useLoading(_handleSendMsg)

async function _handleSendMsg(msgContent: string) {
  const historyId = chatHistory.value?.id

  if (!historyId) {
    throw new Error(`Chat history is not initialized!`)
  }

  const newMsgItem = await chatHistoryMsgTable.createOne({
    chatHistoryId: historyId,
    role: ChatRole.User,
    content: msgContent,
  })

  state.messages.push(newMsgItem)

  state.responseMsg = await chatHistoryMsgTable.createOne({
    chatHistoryId: historyId,
    role: ChatRole.Assistant,
    content: '',
  })

  state.messages.push(state.responseMsg)

  await startChatStream(state.messages.slice(0, -1))
}

const saveResponseMsgItem = debounce(_saveResponseMsgItem, 100)

async function _saveResponseMsgItem() {
  if (!state.responseMsg?.id) return

  await chatHistoryMsgTable.updateOne({
    ...state.responseMsg,
  })
}

async function startChatStream(msgs: IChatHistoryMsgItem[]) {
  const { apiKey, model, baseUrl } =
    promptConfigApi.data.value?.endpointConfig || {}

  if (!baseUrl || !apiKey || !model) {
    throw new Error(`Endpoint config is missing`)
  }

  const ins = new OpenAI({
    baseURL: baseUrl,
    apiKey,
  })

  const resultPromise = createPromise<void>()

  const stream = ins.chat.completions.stream({
    model,
    messages: msgs.map((msg) => {
      const item: ChatCompletionMessageParam = {
        role: msg.role === ChatRole.User ? 'user' : 'assistant',
        content: msg.content,
      }

      return item
    }),
  })

  streamRef = stream

  stream.on('message', (msg) => {
    let content = ''

    if (isArray(msg.content)) {
      msg.content.forEach((item) => {
        switch (item.type) {
          case 'text':
            content += item.text
            break
          case 'refusal':
            content += item.refusal
            break

          case 'image_url':
            content += `![](${item.image_url.url})`
            break

          default:
            throw new Error(`Message type not support`)
        }
      })
    } else {
      content += msg.content || ''
    }

    if (!state.responseMsg) {
      throw new Error(`Response message instance is null`)
    }

    state.responseMsg.content += content

    saveResponseMsgItem()
  })

  stream.on('end', () => {
    state.responseMsg = null
    streamRef = null

    resultPromise.resolve()
  })

  stream.on('error', (err) => {
    resultPromise.reject(err.message)
  })

  return resultPromise.promise
}

function handleAbort() {
  streamRef?.abort()
}
</script>

<template>
  <AutoResizeContainer :width="400">
    <div class="page bg-white">
      <ChatPageHead :icon="promptConfigApi.data.value?.icon" :title="promptConfigApi.data.value?.name" />

      <template v-if="chatHistory">
        <ChatRoom :title="chatHistory.name" :messages="state.messages" :is-processing="handleSendMsg.isLoading"
          @rename-title="updateChatTitle" @send="handleSendMsg" @abort="handleAbort" />
      </template>
    </div>
  </AutoResizeContainer>
</template>

<style lang="scss" scoped></style>
