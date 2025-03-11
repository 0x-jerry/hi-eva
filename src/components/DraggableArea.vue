<script lang='ts' setup>
import { getCurrentWindow } from '@tauri-apps/api/window'

const win = getCurrentWindow()

async function startDrag() {
  await win.startDragging()
}
</script>

<template>
  <div class="cursor-move" @mousedown="startDrag" >
    <slot></slot>
  </div>

</template>

<style lang='less' scoped>
</style>
