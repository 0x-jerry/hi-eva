<script setup lang="ts">
import { useEventListener } from '@vueuse/core'
import { ConfigProvider } from 'tdesign-vue-next'
import NotificationPlugin from 'tdesign-vue-next/es/notification/plugin'
import { onErrorCaptured, onMounted } from 'vue'
import { commands } from './logic/commands'
import { tDesignConfig } from './logic/tdesign'

onMounted(() => {
  commands.applyAppearance()
})

onErrorCaptured((err) => {
  showErrorNotification(err)

  return false
})

useEventListener(window, 'unhandledrejection', (evt) => {
  evt.stopPropagation()

  showErrorNotification(evt.reason)
})

function showErrorNotification(rawErr: unknown) {
  console.error(rawErr)

  const err = rawErr instanceof Error ? rawErr : new Error(String(rawErr))

  NotificationPlugin.error({
    title: err.name,
    content: err.message,
  })
}
</script>

<template>
  <ConfigProvider :global-config="tDesignConfig">
    <RouterView />
  </ConfigProvider>
</template>
