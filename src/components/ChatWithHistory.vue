<script lang="ts" setup>
import { createPromise, isArray, Optional } from '@0x-jerry/utils'
import { useLoading } from '@0x-jerry/vue-kit'
import { watchImmediate } from '@vueuse/core'
import OpenAI from 'openai'
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream.mjs'
import { ChatCompletionMessageParam } from 'openai/resources/index.mjs'
import { reactive, ref } from 'vue'
import { chatHistoryTable, IChatHistoryModel } from '../database/chatHistory'
import {
  chatHistoryMsgTable,
  IChatHistoryMsgItem,
  IChatHistoryMsgModel,
} from '../database/chatHistoryMsg'
import { IEndpointConfigItem } from '../database/endpointConfig'
import { ChatRole } from '../logic/chat'
import ChatRoot from './Chat/ChatRoot.vue'

export interface ChatWithHistoryProps {
  historyId: number
  endpointConfig?: IEndpointConfigItem
}

const props = defineProps<ChatWithHistoryProps>()

const state = reactive({
  selectedText: '',
  pinned: false,
  messages: [] as IChatHistoryMsgItem[],
  responseMsg: null as Optional<IChatHistoryMsgModel>,
})

let streamRef: Optional<ChatCompletionStream>

const chatHistory = ref<IChatHistoryModel>()

let updateDataPromise = null as null | Promise<void>

watchImmediate(
  () => props.historyId,
  () => {
    updateDataPromise = updateHistoryData()
  },
)

async function updateHistoryData() {
  chatHistory.value = await chatHistoryTable.getById(props.historyId)

  await updateMessages()
}

async function updateMessages() {
  const msgs = await chatHistoryMsgTable.getMsgs(props.historyId)

  state.messages = msgs
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
  await updateDataPromise

  const historyId = props.historyId

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

async function saveResponseMsgItem() {
  if (!state.responseMsg?.id) return

  await chatHistoryMsgTable.updateOne({
    ...state.responseMsg,
  })
}

async function startChatStream(msgs: IChatHistoryMsgItem[]) {
  const { apiKey, model, baseUrl } = props.endpointConfig || {}

  if (!baseUrl || !apiKey || !model) {
    throw new Error(`Endpoint config is missing`)
  }

  const ins = new OpenAI({
    baseURL: baseUrl,
    apiKey,
    dangerouslyAllowBrowser: true,
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

  stream.on('end', () => {
    state.responseMsg = null
    streamRef = null

    resultPromise.resolve()
  })

  stream.on('error', (err) => {
    console.error(err)
    resultPromise.reject(err.message)
  })

  for await (const chunk of stream) {
    const firstChoice = chunk.choices[0]

    const content = firstChoice.delta.content

    if (!state.responseMsg) {
      throw new Error(`Response message instance is null`)
    }

    state.responseMsg.content += content
    await saveResponseMsgItem()
  }

  return resultPromise.promise
}

function handleAbort() {
  streamRef?.abort()
}

defineExpose({
  sendMsg: handleSendMsg,
  abortStream: handleAbort,
})
</script>

<template>
  <template v-if="chatHistory">
    <ChatRoot :title="chatHistory.name" :messages="state.messages" :is-processing="handleSendMsg.isLoading"
      @rename-title="updateChatTitle" @send="handleSendMsg" @abort="handleAbort" />
  </template>
  <template v-else>
    <div class="loading">
      Loading ...
    </div>
  </template>
</template>

<style lang="scss" scoped></style>
