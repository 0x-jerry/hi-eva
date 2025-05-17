<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import Icon from './Icon.vue'
import type { Awaitable } from '@vueuse/core'

export interface CloseWindowProps {
  close?: () => Awaitable<void>
}

const props = defineProps<CloseWindowProps>()

const win = getCurrentWindow()

async function closeWindow() {
  if (props.close) {
    await props.close()
    return
  }

  await win.hide()
}
</script>

<template>
  <div
    class="inline-flex items-center cursor-pointer px-1 hover:bg-red hover:text-white"
    @click.stop="closeWindow"
  >
    <Icon class="i-carbon:close" />
  </div>
</template>

<style lang="scss" scoped></style>
