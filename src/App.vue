<script setup lang="ts">
import { ConfigProvider } from 'tdesign-vue-next'
import { onMounted } from 'vue'
import { commands } from './logic/commands'
import { tDesignConfig } from './logic/tdesign'

onMounted(() => {
  commands.applyAppearance()
})
</script>

<template>
  <ConfigProvider :global-config="tDesignConfig">
    <RouterView />
  </ConfigProvider>
</template>
