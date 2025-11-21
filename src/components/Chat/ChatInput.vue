<script setup lang="ts">
import { Button, Textarea } from 'primevue'
import { computed, ref } from 'vue'

export interface ChatInputProps {
  isProcessing?: boolean
}

export type ChatInputEmits = {
  send: [content: string]
  abort: []
}

const props = defineProps<ChatInputProps>()
const emit = defineEmits<ChatInputEmits>()

const text = ref('')

const canSend = computed(() => text.value.trim().length > 0)

function send() {
  if (!canSend.value) return
  if (props.isProcessing) return

  emit('send', text.value.trim())

  text.value = ''
}

function onCtrlEnter(evt: KeyboardEvent) {
  const shouldSend = evt.key === 'Enter' && (evt.ctrlKey || evt.metaKey)
  if (!shouldSend) return

  send()
}

function handleAbort() {
  if (!props.isProcessing) {
    return
  }

  emit('abort')
}
</script>

<template>
  <div class="chat-input">
    <Textarea v-model="text" @keydown="onCtrlEnter" placeholder="Type a message" />

    <div class="actions">
      <Button v-if="isProcessing" @click="handleAbort">Abort</Button>
      <Button v-else @click="send" :disabled="!canSend">Send</Button>
    </div>
  </div>
</template>

<style scoped>
.chat-input {
  display: flex;
  gap: 8px;
  padding: 8px;
  border-top: 1px solid #eee
}

.chat-input textarea {
  flex: 1;
  min-height: 56px;
  padding: 8px;
  resize: vertical
}

.actions {
  display: flex;
  align-items: flex-end
}

.actions button {
  padding: 6px 12px
}
</style>
