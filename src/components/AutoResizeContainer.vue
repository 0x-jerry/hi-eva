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
		Math.min(props.maxWidth || screen.availWidth, el.value.clientWidth)

	const h =
		props.height ??
		Math.min(props.maxHeight || screen.availHeight, el.value.clientHeight)

	const size = new LogicalSize(w, h)
	await win.setSize(size)
	await checkWindowPosition(size)
}

async function checkWindowPosition(size: LogicalSize) {
	const winPos = (await win.outerPosition()).toLogical(devicePixelRatio)

	const gap = 10

	const bottom = screen.availHeight - (winPos.y + size.height)
	if (bottom < gap) {
		const nextHeight = screen.availHeight - (size.height + gap)
		winPos.y = nextHeight
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

<style lang='less' scoped>
.auto-resize-container {
  position: fixed;
}
</style>