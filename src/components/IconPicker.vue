<script setup lang="ts">
import Popover from 'primevue/popover'
import InputText from 'primevue/inputtext'
import { icons } from '@iconify-json/carbon'
import { computed, reactive, ref } from 'vue'
import CarbonIcon from './CarbonIcon.vue'
import { InputGroup, InputGroupAddon } from 'primevue'
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
    <div class="border-(1 solid light-8) rounded size-8 flex items-center justify-center cursor-pointer hover:bg-light-4"
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

        <div class="h-240px overflow-auto">
          <div class="icons text-2xl">
            <span class="inline-block size-8 cursor-pointer hover:bg-light-4 justify-items-center " v-for="name in filteredNames" :key="name" @click="handleSelect(name)">
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
