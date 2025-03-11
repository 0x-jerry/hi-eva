<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTimeoutFn } from '@vueuse/core'
import { promptConfigs, type ToolbarPromptConfig } from '../logic/config'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import Icon from '../components/Icon.vue'
import DraggableArea from '../components/DraggableArea.vue'
import CloseWindow from '../components/CloseWindow.vue'
import { openChatWindow } from '../logic/commands'

const win = getCurrentWindow()

const timeoutHandler = useTimeoutFn(
	async () => {
		hideWindow()
	},
	1_500,
	{
		immediate: false,
	},
)

win.listen('show', () => {
	win.show()

	timeoutHandler.start()
})

win.listen('hide', () => {
	hideWindow()
})

async function hideWindow() {
	timeoutHandler.stop()

	if (!timeoutHandler.isPending.value) {
		await win.hide()
	}
}

async function openChatPage(conf: ToolbarPromptConfig) {
	await hideWindow()
	await openChatWindow(conf.id)
}
</script>

<template>
  <AutoResizeContainer>
    <div
      class="toolbar bg-white flex"
      @mouseleave="timeoutHandler.start()"
      @mouseenter="timeoutHandler.stop()"
    >
      <DraggableArea class="px-1 flex items-center cursor-move bg-gray-1">
        <Icon class="i-carbon:draggable cursor-move" />
      </DraggableArea>
      <div
        v-for="conf in promptConfigs"
        class="flex items-center px-2 hover:bg-gray-2 cursor-pointer"
        @click="openChatPage(conf)"
      >
        <Icon v-if="conf.icon" :class="conf.icon" />
        <span v-else>{{ conf.name }}</span>
      </div>
      <CloseWindow
        class="py-2 px-1 flex items-center hover:bg-gray-2"
      />
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
