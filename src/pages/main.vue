<script setup lang="ts">
import { RouterView, useRoute, useRouter } from 'vue-router'
import Icon from '../components/Icon.vue'

const router = useRouter()
const route = useRoute()

const navButtons = [
  { name: 'Home', path: '/main', icon: 'i-carbon:chat' },
  { name: 'Settings', path: '/main/settings', icon: 'i-carbon:settings' },
  { name: 'Statistics', path: '/main/statistics', icon: 'i-carbon:chart-line-data' }
]
</script>

<template>
  <div class="flex h-screen">
    <div class="nav-col w-60px bg-gray-1 flex flex-col gap-4">
      <div class="flex items-center justify-center pt-1">
        <img src="/logo.svg" alt="logo" class="w-3/4 p-2 rounded-md bg-gray-2 aspect-ratio-square">
      </div>

      <div class="flex-1 flex flex-col text-gray-5">
        <div
          v-for="navButton in navButtons"
          :key="navButton.name"
          class="text-2xl flex items-center justify-center py-2 cursor-pointer hover:text-gray-7"
          :class="{ 'bg-gray-3 text-gray-7': route.path === navButton.path }"
          @click="router.push(navButton.path)"
        >
          <Icon :class="navButton.icon" />
        </div>
      </div>
    </div>

    <div class="content">
      <RouterView />
    </div>

  </div>
</template>

<style>
</style>
