<script setup lang="ts">
import { nextTick, useTemplateRef, watch } from 'vue'
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatInput, { type ChatInputEmits } from './ChatInput.vue'
import ChatMessageList from './ChatMessageList.vue'

export interface ChatRoomProps {
  title: string
  messages: IChatHistoryMsgItem[]
  isProcessing?: boolean
}

export interface ChatRoomEmits extends ChatInputEmits {
  'delete-msg': [msg: IChatHistoryMsgItem]
  'reset-to-msg': [msg: IChatHistoryMsgItem]
  'continue-from-msg': [msg: IChatHistoryMsgItem]
}

const props = defineProps<ChatRoomProps>()

const emit = defineEmits<ChatRoomEmits>()

const containerEl = useTemplateRef('containerEl')

watch(
  () => props.messages.length,
  async () => {
    await nextTick()

    containerEl.value?.scrollTo({
      top: containerEl.value.scrollHeight,
      behavior: 'smooth',
    })
  },
)

function handleSend(content: string) {
  emit('send', content)
}
</script>

<template>
  <div class="chat-root">
    <div class="flex-1 h-0 overflow-auto p-3" ref="containerEl">
      <ChatMessageList :messages="messages" @delete="emit('delete-msg', $event)"
        @reset-to="emit('reset-to-msg', $event)" @continue="emit('continue-from-msg', $event)" />
    </div>

    <ChatInput :is-processing="isProcessing" @send="handleSend" @abort="emit('abort')" />
  </div>
</template>

<style scoped>
.chat-root {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>
