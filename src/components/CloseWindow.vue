<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import Icon from './Icon.vue'

const win = getCurrentWindow()

async function closeWindow() {
	await win.hide()
}
</script>

<template>
  <slot>
    <Icon class="i-carbon:close" @click="closeWindow" />
  </slot>
</template>

<style lang="less" scoped></style>
