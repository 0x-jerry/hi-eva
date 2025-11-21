<script setup lang="ts">
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import Markdown from '../Markdown.vue';

interface ChatMessageProps {
  message: IChatHistoryMsgItem
}

const props = defineProps<ChatMessageProps>()

const roleClass = props.message.role === 'user' ? 'from-user' : 'from-assistant'
</script>

<template>
  <div :class="['chat-message', roleClass]">
    <div class="meta">{{ message.role }}</div>
    <div class="content">
      <Markdown :content="message.content" />
    </div>
  </div>
</template>

<style scoped>
.chat-message {
  padding: 8px;
  border-radius: 6px
}

.from-user {
  align-self: flex-end;
  background: #e6f7ff
}

.from-assistant {
  align-self: flex-start;
  background: #f5f5f5
}

.meta {
  font-size: 12px;
  color: #666
}

.content {
  white-space: pre-wrap;
  margin-top: 4px
}
</style>
