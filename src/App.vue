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
