<script setup lang="ts">
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import Icon from '../Icon.vue'
import Markdown from '../Markdown.vue'

interface ChatMessageProps {
  message: IChatHistoryMsgItem
}

const props = defineProps<ChatMessageProps>()

const roleClass = props.message.role === 'user' ? 'from-user' : 'from-assistant'
const isUser = props.message.role === 'user'
</script>

<template>
  <div :class="['chat-message', roleClass]">
    <div class="meta flex items-center">
      <template v-if="isUser">
        <span class="name">{{ message.role }}</span>
        <Icon class="i-carbon:user-avatar text-2xl" />
      </template>
      <template v-else>
        <Icon class="i-carbon:machine-learning-model text-2xl" />
        <span class="name">{{ message.role }}</span>
      </template>
    </div>

    <div class="content">
      <Markdown :content="message.content" />
    </div>
  </div>
</template>

<style lang="less" scoped>
.chat-message {
  max-width: 90%;
}

.content {
  padding: 8px;
  border-radius: 6px;
  max-width: 100%;
  width: fit-content;
}

.from-user {
  align-self: flex-end;

  .meta {
    justify-content: end;
  }

  .content {
    background: #e6f7ff;
  }
}

.from-assistant {
  align-self: flex-start;

  .content {
    background: #f5f5f5;
  }
}

.meta {
  --uno: text-sm text-gray-5 mb-2;

  .name {
    &::first-letter {
      text-transform: uppercase;
    }
  }
}
</style>
