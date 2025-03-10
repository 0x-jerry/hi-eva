<script lang='ts' setup>
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useResizeObserver } from '@vueuse/core'
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

useResizeObserver(el, resizeWindow)

async function resizeWindow() {
	if (!el.value) return

	const w =
		props.width ??
		Math.min(props.maxWidth || Number.POSITIVE_INFINITY, el.value.clientWidth)

	const h =
		props.height ??
		Math.min(props.maxHeight || Number.POSITIVE_INFINITY, el.value.clientHeight)

	const size = new LogicalSize(w, h)
	await win.setSize(size)
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

<style lang='less' scoped>
.auto-resize-container {
  position: fixed;
  top: 0;
  left: 0;
}
</style>