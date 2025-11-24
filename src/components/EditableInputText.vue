<script lang='ts' setup>
import { Input, type InputProps } from 'tdesign-vue-next'

export interface EditableInputTextProps {
  editable?: boolean
  input?: InputProps
}

defineProps<EditableInputTextProps>()

const modelValue = defineModel<string>()

</script>

<template>
  <template v-if="editable">
    <Input v-bind="input" v-model="modelValue" />
  </template>
  <template v-else>
    {{ modelValue }}
  </template>

</template>

<style lang='less'>
</style>
