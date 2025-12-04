<script setup lang="ts">
import { sleep } from '@0x-jerry/utils'
import { icons } from '@iconify-json/carbon'
import Fuse from 'fuse.js'
import { Input, Popup } from 'tdesign-vue-next'
import { computed, reactive, useTemplateRef, watch } from 'vue'
import { searchAlias } from '../logic/icon'
import CarbonIcon from './CarbonIcon.vue'
import Icon from './Icon.vue'

export interface IconPickerProps {
  disabled?: boolean
}

const props = defineProps<IconPickerProps>()

const vValue = defineModel<string>()

const inputContainerRef = useTemplateRef('inputRef')

const state = reactive({
  searchValue: '',
  visible: false,
})

watch(
  () => state.visible,
  async (visible) => {
    if (!visible) return

    // Delay to trigger input focus, default delay is 250ms
    // https://tdesign.tencent.com/vue-next/components/popup?tab=api
    await sleep(500)

    // https://github.com/Tencent/tdesign-vue-next/issues/6074
    // @ts-expect-error
    inputContainerRef.value?.focus()
  },
)

const iconNames = Object.keys(icons.icons)
const iconSearchableSet = iconNames.map((item) => {
  const alias = searchAlias.find((n) => n.includes(item))

  return {
    name: item,
    alias,
  }
})

const fuse = new Fuse(iconSearchableSet, {
  isCaseSensitive: false,
  keys: ['name', 'alias'],
  threshold: 0.4,
})

const filteredNames = computed(() => {
  const v = state.searchValue.trim()
  if (!v) return iconNames

  const result = fuse.search(v)

  return result.map((item) => item.item.name)
})

function handleSelect(name: string) {
  vValue.value = name
  state.searchValue = ''

  state.visible = false
}
</script>

<template>
  <div class="text-2xl inline-block">
    <Popup v-model="state.visible" trigger="click">
      <div class="icon-picker border-(1 solid light-8) rounded size-8 flex-center cursor-pointer"
        :class="{ 'disabled': disabled }">
        <CarbonIcon :name="vValue || 'data-categorical'" />
      </div>

      <template #content>
        <div class="w-400px p-2">
          <div  class="search mb-4">
            <Input ref="inputRef" class="w-full" v-model="state.searchValue" autofocus>
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
