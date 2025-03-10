<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTimeoutFn } from '@vueuse/core'
import { promptConfigs, type ToolbarPromptConfig } from '../logic/config'
import { useRouter } from 'vue-router'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import Icon from '../components/Icon.vue'
import DraggableArea from '../components/DraggableArea.vue'
import CloseWindow from '../components/CloseWindow.vue'

const router = useRouter()

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

async function hideWindow() {
	await win.hide()
}

async function openChatPage(conf: ToolbarPromptConfig) {
	timeoutHandler.stop()
}
</script>

<template>
  <AutoResizeContainer>
    <div
      class="toolbar bg-white inline-flex items-center overflow-hidden"
      @mouseleave="timeoutHandler.start()"
      @mouseenter="timeoutHandler.stop()"
    >
    <DraggableArea>
      <span class="py-2 px-1 mr-2 cursor-move bg-gray-2" >
        <Icon class="i-carbon:draggable" />
      </span>
    </DraggableArea>
      <div class="flex">
        <div v-for="conf in promptConfigs" class="button" @click="openChatPage(conf)">
          <Icon v-if="conf.icon" :class="conf.icon" />
          <span v-else>{{ conf.name }}</span>
        </div>
      </div>
      <span class="py-2 px-1 flex items-center">
        <CloseWindow />
      </span>
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
