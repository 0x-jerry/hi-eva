<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window'
import CarbonIcon from '../../components/CarbonIcon.vue'
import CloseWindow from '../../components/CloseWindow.vue'
import DraggableArea from '../../components/DraggableArea.vue'
import Icon from '../../components/Icon.vue'
import { commands } from '../../logic/commands'

export interface ChatPageHead {
  icon?: string
  title?: string
}

defineProps<ChatPageHead>()

const vPinned = defineModel('pinned', { default: false })

const win = getCurrentWindow()

async function togglePinWindow() {
  const pinned = !vPinned.value

  vPinned.value = pinned

  await win.setAlwaysOnTop(pinned)
  await commands.setChatPinned({ pinned })
}
</script>

<template>
  <div class="title flex border-(0 b solid gray) h-8 select-none">
    <div class="flex items-center pl-1 cursor-pointer" @click="togglePinWindow">
      <Icon v-if="vPinned" class="i-carbon:pin-filled" />
      <Icon v-else class="i-carbon:pin" />
    </div>
    <div class="border-(0 r solid gray) ml-1 mr-2">&nbsp;</div>
    <DraggableArea class="flex-1 flex items-center gap-1">
      <CarbonIcon v-if="icon" :name="icon" class="text-xl" />
      <span>
        {{ title }}
      </span>
    </DraggableArea>
    <div class="icons h-full">
      <CloseWindow class="h-full" />
    </div>
  </div>

</template>

<style lang="scss" scoped></style>
