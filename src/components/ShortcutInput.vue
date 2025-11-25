<script lang='ts' setup>
import { computed, ref, useTemplateRef } from 'vue'
import { useI18n } from '../composables'

const { t } = useI18n()
const isEditing = ref(false)

const value = defineModel<string>({ default: '' })

const newShortcut = ref('')

const inputEl = useTemplateRef('inputEl')

const emit = defineEmits(['blur', 'change', 'focus'])

const displayText = computed(() => {
  if (isEditing.value) {
    return newShortcut.value || t('common.detecting')
  }

  return value.value || '-'
})

function applyIsEditing(editing: boolean) {
  isEditing.value = editing

  if (isEditing.value) {
    inputEl.value?.focus()
  } else {
    value.value = newShortcut.value

    emit('change')
    inputEl.value?.blur()
  }

  newShortcut.value = ''
}

function handleKeyEvent(evt: KeyboardEvent) {
  if (!isEditing.value) {
    return
  }

  if (evt.key === 'Escape') {
    applyIsEditing(false)
    return
  }

  const ctrl = evt.ctrlKey || evt.metaKey ? 'CommandOrControl' : ''
  const shift = evt.shiftKey ? 'Shift' : ''
  const alt = evt.altKey ? 'Alt' : ''
  const key = evt.key.toUpperCase()

  const keys = [ctrl, shift, alt, key].filter((n) => n)

  newShortcut.value = keys.join('+')
}

function onFocus() {
  applyIsEditing(true)

  emit('focus')
}

function onBlur() {
  applyIsEditing(false)

  emit('blur')
}
</script>

<template>
  <div class="t-input shortcut-input" :class="{ 't-is-focused t-input--focused': isEditing }" @click="applyIsEditing(true)">
    <input ref="inputEl" class="size-0 fixed -top-100px" @blur="onBlur" @focus="onFocus" @keydown="handleKeyEvent" />
    {{ displayText  }}
  </div>
</template>

<style lang='less' scoped>
.shortcut-input {
  --uno: flex items-center justify-center text-gray-6;
}
</style>
