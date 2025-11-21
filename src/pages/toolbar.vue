<script lang="ts" setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { getCurrentWindow } from '@tauri-apps/api/window'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import CarbonIcon from '../components/CarbonIcon.vue'
import { IPromptConfigModel, promptConfigTable } from '../database/promptConfig'
import { commands } from '../logic/commands'
import { WindowEventName } from '../logic/events'

const promptConfigsApi = useAsyncData(promptConfigTable.findAll, [])

promptConfigsApi.load()

const win = getCurrentWindow()

win.listen(WindowEventName.ToolbarShow, () => {
  promptConfigsApi.load()
})

async function openChatPage(conf: IPromptConfigModel) {
  await hideWindow()
  await commands.openChat({ promptId: conf.id.toString() })
}

async function hideWindow() {
  await commands.hideToolbarWindow()
}
</script>

<template>
  <AutoResizeContainer>
    <div class="toolbar bg-white flex h-6">
      <div v-for="conf in promptConfigsApi.data.value" class="flex items-center px-2 hover:bg-gray-2 cursor-pointer"
        @click="openChatPage(conf)">
        <CarbonIcon v-if="conf.icon" :name="conf.icon" />
        <span v-else>{{ conf.name }}</span>
      </div>
    </div>
  </AutoResizeContainer>
</template>

<style lang="scss" scoped></style>
