<script setup lang="ts">
import { ref } from 'vue'

export interface CopyBtnProps {
  content?: string
}

const props = defineProps<CopyBtnProps>()

const isCopied = ref(false)

async function onCopy() {
  if (!props.content) return

  try {
    await navigator.clipboard.writeText(props.content)
    triggerCopied()
  } catch (error) {
    console.error(error)
  }
}

function triggerCopied() {
  isCopied.value = true

  setTimeout(() => {
    isCopied.value = false
  }, 1000)
}
</script>

<template>
  <span class="inline-flex text-gray border-(1 solid current) p-1 rounded cursor-pointer" :class="{ 'text-green-5': isCopied }" @click="onCopy">
    <span v-if="isCopied" class="i-carbon-checkmark"></span>
    <span v-else class="i-carbon-copy"></span>
  </span>
</template>
