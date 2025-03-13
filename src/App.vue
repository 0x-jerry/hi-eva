<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { commands } from './logic/commands'

onMounted(async () => {
  await commands.applyAppearance()
})
</script>

<template>
  <RouterView />
</template>

<style></style>
