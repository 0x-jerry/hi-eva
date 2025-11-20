<script lang='ts' setup>
import type { InputTextProps } from 'primevue/inputtext'
import InputText from 'primevue/inputtext'

export interface EditableInputTextProps {
  editable?: boolean
  input?: InputTextProps
}

defineProps<EditableInputTextProps>()

const modelValue = defineModel<string>()

</script>

<template>
  <template v-if="editable">
    <InputText v-bind="input" v-model="modelValue" />
  </template>
  <template v-else>
    {{ modelValue }}
  </template>

</template>

<style lang='less'>
</style>
