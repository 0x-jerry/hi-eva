<script setup lang="ts">
import { icons } from '@iconify-json/carbon'

export interface CarbonIconProps {
  name?: string
}

defineProps<CarbonIconProps>()
</script>

<template>
  <div class="icon" :data-icon="name">
    <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" :viewBox="`0 0 ${icons.width} ${icons.height}`"
      v-html="icons.icons[name!]?.body"></svg>
  </div>
</template>

<style scoped>
.icon {
  display: inline-block;
  vertical-align: 0;
}

svg {
  display: block;
  margin: auto
}
</style>
