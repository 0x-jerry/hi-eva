<script setup lang="ts">
import { icons } from '@iconify-json/carbon'
import { Input, Popup } from 'tdesign-vue-next'
import { computed, reactive } from 'vue'
import CarbonIcon from './CarbonIcon.vue'
import Icon from './Icon.vue'

export interface IconPickerProps {
  disabled?: boolean
}

const vValue = defineModel<string>()

const state = reactive({
  searchValue: '',
  visible: false,
})

const props = defineProps<IconPickerProps>()

const names = Object.keys(icons.icons)

const filteredNames = computed(() => {
  const v = state.searchValue.trim()
  if (!v) return names

  return names.filter((n) => n.includes(v))
})

async function showPopover() {
  if (props.disabled) {
    return
  }

  state.visible = true
}

function handleSelect(name: string) {
  vValue.value = name
  state.searchValue = ''

  state.visible = false
}
</script>

<template>
  <div class="text-2xl inline-block">
    <Popup v-model="state.visible">
      <div class="icon-picker border-(1 solid light-8) rounded size-8 flex-center cursor-pointer"
        :class="{ 'disabled': disabled }" @click="showPopover">
        <CarbonIcon :name="vValue || 'data-categorical'" />
      </div>

      <template #content>
        <div class="w-400px p-2">
          <div class="search mb-4">
            <Input class="w-full" v-model="state.searchValue">
              <template #suffix-icon>
                <Icon class="i-carbon:search" />
              </template>
            </Input>
          </div>

          <div class="h-200px overflow-auto">
            <div class="icons text-2xl">
              <span class="size-8 cursor-pointer hover:bg-light-6 flex-center rounded" v-for="name in filteredNames"
                :key="name" @click="handleSelect(name)" :title="name">
                <CarbonIcon :name="name" />
              </span>
            </div>
          </div>
        </div>
      </template>
    </Popup>
  </div>
</template>

<style scoped>
.icon-picker {
  --uno: bg-white;

  &:hover {
    --uno: bg-light-6;
  }

  &.disabled {
    cursor: auto;

    &:hover {
      --uno: bg-white;
    }
  }
}

.icons {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 0.75rem;
}
</style>
