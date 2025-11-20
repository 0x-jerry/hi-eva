<script lang="ts" setup>
import { onMounted } from 'vue'
import AutoResizeContainer from '../components/AutoResizeContainer.vue'
import CarbonIcon from '../components/CarbonIcon.vue'
import type { PromptConfig } from '../composables'
import { commands } from '../logic/commands'
import { promptConfigs } from '../logic/config'

onMounted(async () => {
  await commands.applyAppearance()
})

async function openChatPage(conf: PromptConfig) {
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

<style lang="scss" scoped></style>
