<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';

const settings = [
  {
    label: '通用设置',
    path: 'common'
  },
  {
    label: 'AI 设置',
    path: 'ai'
  },
  {
    label: '关于',
    path: 'about'
  },
]

</script>
<template>
  <div class="page">
    <div class="sidebar">
      <RouterLink v-for="conf in settings" :to="`/settings/${conf.path}`">
        {{ conf.label }}
      </RouterLink>
    </div>
    <div class="content">
      <div class="section">
        <RouterView></RouterView>
      </div>
    </div>
  </div>
</template>
<style lang="less" scoped></style>