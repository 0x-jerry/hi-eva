<script setup lang="ts">
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatHeader from './ChatHeader.vue'
import ChatInput, { type ChatInputEmits } from './ChatInput.vue'
import ChatMessageList from './ChatMessageList.vue'

export interface ChatRoomProps {
  title: string
  messages: IChatHistoryMsgItem[]
  isProcessing?: boolean
}

export type ChatRoomEmits = ChatInputEmits & {
  renameTitle: [newTitle: string]
}

defineProps<ChatRoomProps>()

const emit = defineEmits<ChatRoomEmits>()

function handleSend(content: string) {
  emit('send', content)
}

function renameChat(newName: string) {
  emit('renameTitle', newName)
}
</script>

<template>
  <div class="chat-root">
    <ChatHeader :name="title" @rename="renameChat" />

    <ChatMessageList :messages="messages" />

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
