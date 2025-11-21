<script lang='ts' setup generic="T">
import { onClickOutside } from '@vueuse/core'
import { ref, toRaw, useTemplateRef } from 'vue'

export interface ClickToEditProps<Value> {
  value?: Value
}

export type ClickToEditEmits<Value> = {
  ok: [newValue: Value]
  cancel: []
  toggle: [editing: boolean]
}

const props = defineProps<ClickToEditProps<T>>()

const emit = defineEmits<ClickToEditEmits<T>>()

const currentValue = ref(structuredClone(toRaw(props.value)))

const isEditing = ref(false)

const rootEl = useTemplateRef('rootEl')

onClickOutside(rootEl, () => {
  if (!isEditing.value) {
    return
  }

  ok()
})

function updateCurrentValue() {
  currentValue.value = structuredClone(toRaw(props.value))
}

function switchToEdit() {
  if (isEditing.value) {
    return
  }

  updateCurrentValue()

  isEditing.value = true
  emit('toggle', isEditing.value)
}

function ok() {
  isEditing.value = false
  emit('toggle', isEditing.value)

  const clonedValue = structuredClone(toRaw(currentValue.value))
  emit('ok', clonedValue)
}

function cancel() {
  isEditing.value = false
  emit('toggle', isEditing.value)

  updateCurrentValue()

  emit('cancel')
}

function updateValue(newValue?: T | ((old: T) => T)) {
  if (typeof newValue === 'function') {
    // @ts-expect-error
    currentValue.value = newValue(currentValue.value)
  } else {
    currentValue.value = newValue
  }
}
</script>

<template>
  <div ref="rootEl" class="cursor-pointer w-full" @click="switchToEdit" @keydown.enter.stop="ok" @keydown.escape.stop="cancel">
    <template v-if="isEditing">
      <slot name="edit" :ok="ok" :cancel="cancel" :value="currentValue" :update="updateValue"></slot>
    </template>
    <template v-else>
      <slot :value="value" ></slot>
    </template>
  </div>

</template>

<style lang='less'></style>
