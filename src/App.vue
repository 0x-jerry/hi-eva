<script setup lang="ts">
import { onMounted } from 'vue'
import { commands } from './logic/commands'

onMounted(() => {
  commands.applyAppearance()
})
</script>

<template>
  <RouterView />
</template>

<style>
html, body {
  --uno: text-gray-7;
}
</style>
