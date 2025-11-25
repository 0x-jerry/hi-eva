<script setup lang="ts">
import { Button, Textarea, TextareaValue } from 'tdesign-vue-next'
import { computed, ref } from 'vue'
import { useI18n } from '../../composables'

export interface ChatInputProps {
  isProcessing?: boolean
}

export type ChatInputEmits = {
  send: [content: string]
  abort: []
}

const props = defineProps<ChatInputProps>()
const emit = defineEmits<ChatInputEmits>()
const { t } = useI18n()

const text = ref('')

const canSend = computed(() => text.value.trim().length > 0)

function send() {
  if (!canSend.value) return
  if (props.isProcessing) return

  emit('send', text.value.trim())

  text.value = ''
}

function onCtrlEnter(_: TextareaValue, opt: { e: KeyboardEvent }) {
  const evt = opt.e
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
    <Textarea style="resize: none;" v-model="text" @keydown="onCtrlEnter" :placeholder="t('chat.typeMessage')" :rows="3" />

    <div class="actions">
      <Button class="h-full" v-if="isProcessing" @click="handleAbort">{{ t('common.abort') }}</Button>
      <Button class="h-full" v-else @click="send" :disabled="!canSend">{{ t('common.send') }}</Button>
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
</style>
