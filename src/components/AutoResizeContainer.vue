<script lang='ts' setup>
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useResizeObserver } from '@vueuse/core'
import { ref } from 'vue'

export interface AutoResizeContainerProps {
	maxWidth?: number
	maxHeight?: number
}

const props = defineProps<AutoResizeContainerProps>()

const el = ref<HTMLElement>()
const win = getCurrentWindow()

useResizeObserver(el, resizeWindow)

async function resizeWindow() {
	if (!el.value) return

	const w = Math.min(
		props.maxWidth || Number.POSITIVE_INFINITY,
		el.value.clientWidth,
	)

	const h = Math.min(
		props.maxHeight || Number.POSITIVE_INFINITY,
		el.value.clientHeight,
	)

	const size = new LogicalSize(w, h)
	await win.setSize(size)
}
</script>

<template>
  <div ref="el" class="auto-resize-container">
    <slot></slot>
  </div>

</template>

<style lang='less' scoped>
.auto-resize-container {
  width: max-content;
  height: max-content;
}
</style>