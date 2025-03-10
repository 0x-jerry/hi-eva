<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTimeoutFn } from '@vueuse/core'
import { reactive } from 'vue'
import SelectButton from 'primevue/selectbutton'
import { promptConfigs } from '../logic/config'
import { useRouter } from 'vue-router'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import Icon from '../components/Icon.vue'

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

win.listen('show', () => {
	win.show()

	timeoutHandler.start()
})

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
  <AutoResizeContainer>
    <div
      class="toolbar bg-white inline-flex items-center"
      @mouseleave="timeoutHandler.start()"
      @mouseenter="timeoutHandler.stop()"
    >
      <span class="py-2 px-1 mr-2 cursor-move bg-gray-2" @mousedown="startDrag">
        <Icon class="i-carbon:draggable" />
      </span>
      <div class="flex">
        <SelectButton
          v-model="state.selectedPrompt"
          :options="promptConfigs"
          option-label="name"
          option-value="id"
          @change="openChatPage"
        />
      </div>
      <span class="cursor-pointer py-2 px-1 flex items-center">
        <Icon class="i-carbon:close" @click="hideWindow" />
      </span>
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
