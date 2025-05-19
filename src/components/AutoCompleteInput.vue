<script lang='ts' generic="T extends any" setup>
import AutoComplete, {
  type AutoCompleteCompleteEvent,
  AutoCompleteSlots,
} from 'primevue/autocomplete'
import { shallowRef } from 'vue'
import Fuse, { type IFuseOptions } from 'fuse.js'
import { watchImmediate } from '@vueuse/core'
import { isString } from '@0x-jerry/utils'

export interface AutoCompleteInputProps<T> {
  items?: T[]
  optionLabel?: keyof T | ({} & string)
  fuse?: IFuseOptions<NoInfer<T>>
}

const slots = defineSlots<AutoCompleteSlots>()
const {
  items,
  fuse: option,
  ...restProps
} = defineProps<AutoCompleteInputProps<T>>()

const vValue = defineModel<string>()

const suggestions = shallowRef<T[]>([])

const fuse = new Fuse<T>([], option)

watchImmediate(
  () => items,
  () => {
    fuse.setCollection(items || [])
  },
)

function onSearchEndpoint(event: AutoCompleteCompleteEvent) {
  const q = event.query

  suggestions.value = fuse.search(q).map((r) => r.item)
}

function updateValue(newValue: Record<string, string> | string) {
  if (isString(newValue)) {
    vValue.value = newValue
  } else {
    vValue.value = newValue[restProps.optionLabel]
  }
}
</script>

<template>
  <AutoComplete v-bind="restProps" :model-value="vValue" @update:model-value="updateValue" :suggestions="suggestions" @complete="onSearchEndpoint" >
    <template v-for="name in slots" #[name]="data">
      <slot :name="name" v-bind="data || {}"></slot>
    </template>
  </AutoComplete>
</template>

<style lang='scss' scoped>
</style>
