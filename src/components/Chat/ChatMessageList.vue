<script setup lang="ts">
import { MessagePlugin } from 'tdesign-vue-next'
import { nextTick, ref, watch } from 'vue'
import { useI18n } from '../../composables'
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import ChatMessage, { ChatMessageEmits } from './ChatMessage.vue'

interface ChatMessageListProps {
  messages: IChatHistoryMsgItem[]
}

export interface ChatMessageListEmits extends Omit<ChatMessageEmits, 'copy'> {}

const props = defineProps<ChatMessageListProps>()

const emit = defineEmits<ChatMessageListEmits>()
const container = ref<HTMLElement | null>(null)

const { t } = useI18n()

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

async function handleCopy(msg: IChatHistoryMsgItem) {
  const content = msg.content
  await navigator.clipboard.writeText(content)
  MessagePlugin.success(t('common.copied'), 1000)
}
</script>

<template>
  <div class="message-list" ref="container">
    <ChatMessage v-for="m in messages.toReversed()" :key="m.id" :message="m" @copy="handleCopy"
      @reset-to="emit('reset-to', $event)" @delete="emit('delete', $event)" @continue="emit('continue', $event)" />
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
