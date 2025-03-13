<script lang="ts" setup>
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import CloseWindow from '../components/CloseWindow.vue'
import DraggableArea from '../components/DraggableArea.vue'
import Icon from '../components/Icon.vue'
import { type ToolbarPromptConfig, promptConfigs } from '../logic/config'
import CarbonIcon from '../components/CarbonIcon.vue'
import { commands } from '../logic/commands'
import { getCurrentWindow } from '@tauri-apps/api/window'

let win = getCurrentWindow()

async function openChatPage(conf: ToolbarPromptConfig) {
  await win.hide()
  await commands.openChat({ promptId: conf.id })
}
</script>

<template>
  <AutoResizeContainer>
    <div class="toolbar bg-white flex">
      <DraggableArea class="px-1 flex items-center cursor-move bg-gray-1">
        <Icon class="i-carbon:draggable cursor-move" />
      </DraggableArea>
      <div v-for="conf in promptConfigs" class="flex items-center px-2 hover:bg-gray-2 cursor-pointer"
        @click="openChatPage(conf)">
        <CarbonIcon v-if="conf.icon" :name="conf.icon" />
        <span v-else>{{ conf.name }}</span>
      </div>
      <CloseWindow class="py-2 px-1 flex items-center hover:bg-gray-2" />
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
