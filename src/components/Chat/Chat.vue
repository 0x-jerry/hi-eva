<script setup lang="ts">
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatHeader from './ChatHeader.vue'
import ChatInput, { type ChatInputEmits } from './ChatInput.vue'
import ChatMessageList from './ChatMessageList.vue'

export interface ChatProps {
  title: string
  messages: IChatHistoryMsgItem[]
}

export type ChatEmits = ChatInputEmits & {
  renameTitle: [newTitle: string]
}

const props = defineProps<ChatProps>()

const emit = defineEmits<ChatEmits>()

async function handleSend(content: string) {
  emit('send', content)
}

async function renameChat(newName: string) {
  emit('renameTitle', newName)
}
</script>

<template>
  <div class="chat-root">
    <ChatHeader :name="title ?? 'New Chat'" @rename="renameChat" />

    <ChatMessageList :messages="messages" />

    <ChatInput @send="handleSend" />
  </div>
</template>

<style scoped>
.chat-root {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>
