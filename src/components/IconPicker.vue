<script setup lang="ts">
import { icons } from '@iconify-json/carbon'
import { InputGroup, InputGroupAddon } from 'primevue'
import InputText from 'primevue/inputtext'
import Popover from 'primevue/popover'
import { computed, reactive, ref } from 'vue'
import CarbonIcon from './CarbonIcon.vue'
import Icon from './Icon.vue'

export interface IconPickerProps {
  disabled?: boolean
}

const vValue = defineModel<string>()

const state = reactive({
  searchValue: '',
})

const props = defineProps<IconPickerProps>()

const popoverEl = ref<InstanceType<typeof Popover>>()

const names = Object.keys(icons.icons)

const filteredNames = computed(() => {
  const v = state.searchValue.trim()
  if (!v) return names

  return names.filter((n) => n.includes(v))
})

async function showPopover(e: Event) {
  if (props.disabled) {
    return
  }

  popoverEl.value?.show(e)
}

function handleSelect(name: string) {
  vValue.value = name
  state.searchValue = ''

  popoverEl.value?.hide()
}
</script>

<template>
  <div class="text-2xl inline-block">
    <div class="border-(1 solid light-8) rounded size-8 flex-center cursor-pointer hover:bg-light-6"
      @click="showPopover">
      <CarbonIcon :name="vValue || 'data-categorical'" />
    </div>

    <Popover ref="popoverEl">
      <div class="w-400px p-2">
        <div class="search mb-4">
          <InputGroup>
            <InputText class="w-full" type="text" v-model="state.searchValue" />
            <InputGroupAddon>
              <Icon class="i-carbon:search" />
            </InputGroupAddon>
          </InputGroup>
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
    </Popover>
  </div>
</template>

<style scoped>
.icons {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 0.75rem;
}
</style>
