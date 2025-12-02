<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatMessage from './ChatMessage.vue'

interface ChatMessageListProps {
  messages: IChatHistoryMsgItem[]
}

const props = defineProps<ChatMessageListProps>()

const container = ref<HTMLElement | null>(null)

watch(
  () => props.messages.length,
  async () => {
    await nextTick()

    container.value?.scrollTo({
      top: container.value.scrollHeight,
      behavior: 'smooth',
    })
  },
)
</script>

<template>
  <div class="message-list" ref="container">
    <ChatMessage v-for="m in messages.toReversed()" :key="m.id" :message="m" />
  </div>
</template>

<style scoped>
.message-list {
  height: max-content;
  display: flex;
  flex-direction: column-reverse;
  gap: 8px;
}
</style>
