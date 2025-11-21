<script setup lang="ts">
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatInput, { type ChatInputEmits } from './ChatInput.vue'
import ChatMessageList from './ChatMessageList.vue'

export interface ChatRoomProps {
  title: string
  messages: IChatHistoryMsgItem[]
  isProcessing?: boolean
}

export type ChatRoomEmits = ChatInputEmits

defineProps<ChatRoomProps>()

const emit = defineEmits<ChatRoomEmits>()

function handleSend(content: string) {
  emit('send', content)
}
</script>

<template>
  <div class="chat-root flex flex-col">
    <div class="flex-1 h-0">
      <ChatMessageList :messages="messages" />
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
