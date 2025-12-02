<script setup lang="ts">
import { Button, Textarea, TextareaValue } from 'tdesign-vue-next'
import { computed, ref } from 'vue'
import { useI18n } from '../../composables'
import Icon from '../Icon.vue'

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
    <Textarea style="resize: none;" v-model="text" @keydown="onCtrlEnter" :placeholder="t('chat.typeMessage')"
      :rows="3" />

    <div class="actions">
      <Button class="h-full relative text-2xl" v-if="isProcessing" @click="handleAbort" :title="t('common.abort')">
        <div class="spinning-effect"></div>
        <Icon class="i-carbon:stop-filled-alt" />
      </Button>
      <Button class="h-full text-2xl" v-else @click="send" :disabled="!canSend" :title="t('common.send')">
        <Icon class="i-carbon:send-alt-filled" />
      </Button>
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

.spinning-effect {
  --size: 40px;

  position: absolute;
  z-index: 0;
  top: 50%;
  left: 50%;
  width: var(--size);
  aspect-ratio: 1;
  transform: translate(-50%, -50%);

  &::before {
    --border: 4px;
    --color: white;
    position: absolute;
    display: block;
    top: 0;
    left: 0;
    content: '';
    width: 100%;
    aspect-ratio: 1;
    border-radius: 50%;
    background:
      radial-gradient(farthest-side, var(--color) 94%, #0000) top/var(--border) var(--border) no-repeat,
      conic-gradient(#0000 30%, var(--color));

    mask: radial-gradient(farthest-side, #0000 calc(100% - var(--border)), #000 0);
    animation: l13 1s infinite linear;
  }

}

@keyframes l13 {
  100% {
    transform: rotate(1turn)
  }
}

.actions {
  display: flex;
  align-items: flex-end
}
</style>
