<script setup lang="ts">
import { ref } from 'vue'

export interface ChatHeaderProps {
  name: string
}

export type ChatHeaderEmits = {
  rename: [newName: string]
}

const props = defineProps<ChatHeaderProps>()
const emit = defineEmits<ChatHeaderEmits>()

const editing = ref(false)
const localName = ref(props.name)

function startEdit() {
  localName.value = props.name
  editing.value = true
}

function save() {
  emit('rename', localName.value.trim() || 'Untitled')
  editing.value = false
}

function cancel() {
  editing.value = false
}
</script>

<template>
  <header class="chat-header">
    <div class="title" v-if="!editing">{{ name }}</div>

    <input v-if="editing" v-model="localName" class="title-input" />

    <div class="actions">
      <button v-if="!editing" @click="startEdit">Rename</button>
      <button v-if="editing" @click="save">Save</button>
      <button v-if="editing" @click="cancel">Cancel</button>
    </div>
  </header>
</template>

<style scoped>
.chat-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-bottom: 1px solid var(--el-border-color, #e6e6e6);
}

.title {
  font-weight: 600
}

.title-input {
  padding: 4px 8px
}

.actions button {
  margin-left: 8px
}
</style>
