<script setup lang="ts">
import { useEventListener } from '@vueuse/core'
import { remove } from 'lodash-es'
import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs'
import Button from 'primevue/button'
import ContextMenu from 'primevue/contextmenu'
import type { MenuItem } from 'primevue/menuitem'
import Textarea from 'primevue/textarea'
import { nextTick, reactive, ref } from 'vue'
import { chatHistoryTable } from '../../database/chatHistory'
import { chatHistoryMsgTable } from '../../database/chatHistoryMsg'
import CopyBtn from '../CopyBtn.vue'
import Markdown from '../Markdown.vue'
import { chatWithPrompt } from './chat'
import type { ChatHistory, ChatMessageWithDb } from './types'

export interface ChatMessagesProps {
  promptId: string
}

const props = defineProps<ChatMessagesProps>()

const chat = defineModel<ChatHistory>()

const emit = defineEmits(['click-title'])

const menuRef = ref<InstanceType<typeof ContextMenu>>()

const contextMenuState = reactive({
  selectMsg: null as ChatMessageWithDb | null,
})

const scrollEl = ref<HTMLDivElement>()

let requestAbort: AbortController | null = null

const state = reactive({
  autoScrollToBottom: true,
  reply: '',
  isThinking: false,
  isReplying: false,
})

useEventListener(scrollEl, 'wheel', (_evt) => {
  const el = scrollEl.value
  if (!el) return

  const gap = 10

  state.autoScrollToBottom =
    el.scrollTop + el.clientHeight + gap >= el.scrollHeight
})

async function scrollToBottom() {
  if (!state.autoScrollToBottom) {
    return
  }

  await nextTick()
  scrollEl.value?.scrollTo({
    top: scrollEl.value.scrollHeight,
    behavior: 'instant',
  })
}

async function ensureChatPersisted() {
  if (!chat.value) return
  // already persisted when id is a number-like string
  if (chat.value.id) return

  const title = chat.value.name || String(chat.value.messages?.[0]?.content ?? '') || 'New Chat'

  const created = await chatHistoryTable.createOne({
    title,
    model: '',
    promptConfigId: Number(props.promptId) || 0,
  })

  if (created) {
    chat.value.id = String(created.id)
  }
}

function handleKeydown(evt: KeyboardEvent) {
  if (evt.key === 'Enter') {
    evt.preventDefault()
    onSend()
  }
}

async function onSend() {
  if (!chat.value) return
  if (state.isThinking || state.isReplying) return

  const reply = state.reply.trim()
  if (!reply) return
  state.reply = ''

  const userMsg: ChatMessageWithDb = { role: 'user', content: reply }
  chat.value.messages.push(userMsg)
  // persist user message (ensure chat record exists)
  try {
    await ensureChatPersisted()
    const created = await chatHistoryMsgTable.createOne({
      role: 'user',
      content: reply,
      chatHistoryId: Number(chat.value.id),
    })
    if (created?.id) {
      const last = chat.value.messages.at(-1)
      if (last) last.__dbId = created.id
    }
  } catch (e) {
    // ignore persistence errors for now
  }
  scrollToBottom()

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

  // persist assistant placeholder
  try {
    await ensureChatPersisted()
    const created = await chatHistoryMsgTable.createOne({
      role: 'assistant',
      content: '',
      chatHistoryId: Number(chat.value.id),
    })
    const msg = chat.value.messages.at(-1)
    if (created?.id && msg) msg.__dbId = created.id
  } catch (e) {
    // ignore persistence errors
  }

  try {
    const respStream = await chatWithPrompt(messages, {
      promptId: props.promptId,
    })

    state.isThinking = false
    state.isReplying = true

    // biome-ignore lint/style/noNonNullAssertion: false positive
    const msg = chat.value.messages.at(-1)!

    requestAbort = respStream.controller

    for await (const chunkItem of respStream) {
      const chunkContent = chunkItem.choices.at(0)?.delta.content || ''
      msg.content += chunkContent

      // update DB record if exists
      try {
        const dbId = msg.__dbId
        if (dbId) {
          await chatHistoryMsgTable.updateOne({
            id: dbId,
            role: 'assistant',
            content: String(msg.content),
            chatHistoryId: Number(chat.value.id),
          })
        }
      } catch (e) {
        // ignore update errors
      }

      scrollToBottom()
    }
  } catch (error) {
    const lastMsg = chat.value.messages.at(-1)

    if (lastMsg) {
      lastMsg.content += String(error)
      // persist error to DB if possible
      try {
        const dbId = lastMsg.__dbId
        if (dbId) {
          await chatHistoryMsgTable.updateOne({
            id: dbId,
            role: 'assistant',
            content: String(lastMsg.content),
            chatHistoryId: Number(chat.value.id),
          })
        }
      } catch (e) {
        // ignore
      }
    }
  }

  requestAbort = null

  state.isReplying = false
}

function stopChatStream() {
  if (requestAbort?.signal.aborted === false) {
    requestAbort.abort()
    state.isReplying = false
    requestAbort = null
  }
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

      // delete DB rows for removed messages
      const removed = chat.value.messages.slice(idx)
      for (const m of removed) {
        const dbId = m.__dbId
        if (dbId) {
          try {
            chatHistoryMsgTable.deleteById(dbId)
          } catch (e) {
            // ignore
          }
        }
      }

      chat.value.messages = chat.value.messages.slice(0, idx)

      continueChat()
    },
  },
  {
    label: '删除',
    icon: 'pi pi-trash',
    command() {
      stopChatStream()

      state.isReplying = false
      state.isThinking = false

      const target = contextMenuState.selectMsg
      if (target) {
        const dbId = target.__dbId
        if (dbId) {
          try {
            chatHistoryMsgTable.deleteById(dbId)
          } catch (e) {
            // ignore
          }
        }
      }

      remove(
        chat.value?.messages || [],
        (msg) => msg === contextMenuState.selectMsg,
      )
    },
  },
]

function showContextmenu(event: Event, msg: ChatMessageWithDb) {
  contextMenuState.selectMsg = msg
  menuRef.value?.show(event)
}

defineExpose({
  continueChat,
})
</script>

<template>
  <div class="flex flex-col bg-white min-h-100px max-h-600px">
    <div ref="scrollEl" class="chat-msgs overflow-auto p-4">
      <div class="chat-msg-item flex flex-col mb-4" v-for="(msg, _idx) in chat?.messages">
        <div class="role-box flex items-center text-(2xl)">
          <div class="role flex flex-1 w-0 truncate">
            <span v-if="msg.role === 'user'" class="i-carbon-user-avatar-filled-alt text-blue-4"></span>
            <span v-else class="i-carbon-machine-learning text-rose-4"></span>
          </div>
          <div class="role-tools flex gap-1">
            <CopyBtn class="text-sm" :content="String(msg.content)" />
          </div>
        </div>
        <div class="chat-msg-content ml-3 mt-2 p-2 bg-light-5 rounded border-(1 solid gray-2)"
          @contextmenu.prevent="showContextmenu($event, msg)">
          <template v-if="msg.content">
            <Markdown :content="String(msg.content)" />
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
      <Button severity="danger" v-if="state.isReplying || state.isThinking" @click="stopChatStream">
        <span class="i-carbon-pause-outline text-3xl"></span>
      </Button>
      <Button v-else @click=onSend>
        <span class="i-carbon-send-alt text-3xl"></span>
      </Button>
    </div>
  </div>
</template>

<style scoped>
.chat-msgs {
  display: flex;
  flex-direction: column;
}

.chat-msg-item {
  .role-tools {
    opacity: 0;
    transition: all ease .4s;
  }

  &:hover {
    .role-tools {
      opacity: 1;
    }
  }
}
</style>
