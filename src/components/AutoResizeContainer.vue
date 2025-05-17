<script lang='ts' setup>
import { LogicalSize, getCurrentWindow } from '@tauri-apps/api/window'
import { useResizeObserver } from '@vueuse/core'
import { debounce } from 'lodash-es'
import { computed, ref } from 'vue'

export interface AutoResizeContainerProps {
  width?: number
  height?: number
  maxWidth?: number
  maxHeight?: number
}

const props = defineProps<AutoResizeContainerProps>()

const el = ref<HTMLElement>()
const win = getCurrentWindow()

const resizeWindow = debounce(_resizeWindow, 100)

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
  await checkWindowPosition(size)
}

async function checkWindowPosition(size: LogicalSize) {
  const winPos = (await win.outerPosition()).toLogical(devicePixelRatio)
  let shouldUpdate = false

  const gap = 10

  const bottom = screen.availHeight - (winPos.y + size.height)

  const right = screen.availWidth - (winPos.x + size.width)

  if (bottom < gap) {
    const nextY = screen.availHeight - (size.height + gap)
    winPos.y = nextY
    shouldUpdate = true
  }

  if (right < gap) {
    const nextX = screen.availWidth - (size.width + gap)
    winPos.x = nextX
    shouldUpdate = true
  }

  if (shouldUpdate) {
    await win.setPosition(winPos)
  }
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
