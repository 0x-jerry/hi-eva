<script lang="ts" setup>
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import { type ToolbarPromptConfig, promptConfigs } from '../logic/config'
import CarbonIcon from '../components/CarbonIcon.vue'
import { commands } from '../logic/commands'
import { onMounted } from 'vue'

onMounted(async () => {
  await commands.applyAppearance()
})

async function openChatPage(conf: ToolbarPromptConfig) {
  await hideWindow()
  await commands.openChat({ promptId: conf.id })
}

async function hideWindow() {
  await commands.hideToolbarWindow()
}
</script>

<template>
  <AutoResizeContainer>
    <div class="toolbar bg-white flex h-6">
      <div v-for="conf in promptConfigs" class="flex items-center px-2 hover:bg-gray-2 cursor-pointer"
        @click="openChatPage(conf)">
        <CarbonIcon v-if="conf.icon" :name="conf.icon" />
        <span v-else>{{ conf.name }}</span>
      </div>
    </div>
  </AutoResizeContainer>
</template>

<style lang="less" scoped></style>
