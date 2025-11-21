<script lang='ts' setup>
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useResizeObserver } from '@vueuse/core'
import { debounce } from 'lodash-es'
import { computed, ref } from 'vue'
import { checkWindowPosition } from '../utils/win'

export interface AutoResizeContainerProps {
  width?: number
  height?: number
  maxWidth?: number
  maxHeight?: number
}

const props = defineProps<AutoResizeContainerProps>()

const el = ref<HTMLElement>()
const win = getCurrentWindow()

const resizeWindow = debounce(_resizeWindow, 100, { trailing: true })

useResizeObserver(el, resizeWindow)

async function _resizeWindow() {
  if (!el.value) return

  const w =
    props.width ??
    Math.min(props.maxWidth || screen.availWidth, el.value.clientWidth)

  const h =
    props.height ??
    Math.min(props.maxHeight || screen.availHeight, el.value.clientHeight)

  const minSise = { w: 20, h: 20 }

  const size = new LogicalSize(Math.max(w, minSise.w), Math.max(h, minSise.h))
  await win.setSize(size)
  await checkWindowPosition()
}

const style = computed(() => {
  return {
    width: `${props.width}px`,
    height: `${props.height}px`,
    maxWidth: `${props.maxWidth}px`,
    maxHeight: `${props.maxHeight}px`,
  }
})
</script>

<template>
  <div ref="el" class="auto-resize-container" :style="style">
    <slot></slot>
  </div>

</template>

<style lang='scss' scoped>
.auto-resize-container {
  position: fixed;
}
</style>
