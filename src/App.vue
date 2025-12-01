<script setup lang="ts">
import { ConfigProvider } from 'tdesign-vue-next'
import NotificationPlugin from 'tdesign-vue-next/es/notification/plugin'
import { onErrorCaptured, onMounted } from 'vue'
import { commands } from './logic/commands'
import { tDesignConfig } from './logic/tdesign'

onMounted(() => {
  commands.applyAppearance()
})

onErrorCaptured((err) => {
  NotificationPlugin.error({
    title: err.name,
    content: err.message,
  })
})
</script>

<template>
  <ConfigProvider :global-config="tDesignConfig">
    <RouterView />
  </ConfigProvider>
</template>
