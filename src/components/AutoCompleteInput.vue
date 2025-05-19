<script lang='ts' generic="T extends any" setup>
import AutoComplete, {
  type AutoCompleteProps,
  type AutoCompleteCompleteEvent,
  AutoCompleteSlots,
} from 'primevue/autocomplete'
import { shallowRef } from 'vue'
import Fuse, { type FuseResult, type IFuseOptions } from 'fuse.js'
import { get } from 'lodash-es'
import { isString } from '@0x-jerry/utils'

export interface AutoCompleteInputProps<T> extends AutoCompleteProps {
  items: T[]
  optionValue?: keyof T | ({} & string)
  fuse?: IFuseOptions<NoInfer<T>>
}

const slots = defineSlots<AutoCompleteSlots>()
const {
  items,
  fuse: option,
  optionValue = 'value',
  ...restProps
} = defineProps<AutoCompleteInputProps<T>>()

const vValue = defineModel<string>()

const suggestions = shallowRef<FuseResult<T>[]>([])

const fuse = new Fuse<T>(items, option)

function onSearchEndpoint(event: AutoCompleteCompleteEvent) {
  const q = event.query

  suggestions.value = fuse.search(q)
}

function updateValue(value: Record<string, string> | string) {
  if (isString(value)) {
    vValue.value = value
  } else {
    vValue.value = get(value, [optionValue])
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
