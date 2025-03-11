<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import Icon from './Icon.vue'

const win = getCurrentWindow()

async function closeWindow() {
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

<style lang="less" scoped></style>
