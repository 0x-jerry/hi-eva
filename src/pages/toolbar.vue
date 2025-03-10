<script lang='ts' setup>
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useTimeoutFn } from '@vueuse/core'
import { reactive, ref } from 'vue'
import SelectButton from 'primevue/selectbutton'
import { promptConfigs } from '../logic/config'
import { useRouter } from 'vue-router'

const router = useRouter()

const state = reactive({
	selectedPrompt: '',
})

const win = getCurrentWindow()

const timeoutHandler = useTimeoutFn(
	() => {
		hideWindow()
	},
	1000,
	{
		immediate: false,
	},
)

const el = ref<HTMLElement>()

win.listen('show', () => {
	win.show()

	resizeWindow()
	timeoutHandler.start()
})

async function resizeWindow() {
	if (!el.value) return
	const w = el.value.clientWidth
	const h = el.value.clientHeight

	const size = new LogicalSize(w, h)
	await win.setSize(size)
}

async function startDrag() {
	await win.startDragging()
}

async function hideWindow() {
	await win.hide()
}

async function openChatPage() {
	timeoutHandler.stop()
	router.push({
		path: '/chat',
		query: {
			promptId: state.selectedPrompt,
			// todo, get selected text
			selectedText: '',
		},
	})
}
</script>

<template>
  <div class="toolbar bg-white inline-flex items-center w-max" ref="el" @mouseleave="timeoutHandler.start()"
    @mouseenter="timeoutHandler.stop()">
    <span class="py-2 px-1 mr-2 cursor-move bg-gray-2" @mousedown="startDrag">
      <span class="icon i-carbon:draggable" />
    </span>
    <div class="flex">
      <SelectButton v-model="state.selectedPrompt" :options="promptConfigs" option-label="name" option-value="id" @change="openChatPage" />
    </div>
    <span class="cursor-pointer py-2 px-1 flex items-center">
      <span class="icon i-carbon:close" @click="hideWindow" />
    </span>
  </div>
</template> This is t

<style lang='less' scoped></style>