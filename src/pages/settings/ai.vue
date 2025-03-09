<script lang='ts' setup>
import { useLocalStorage } from '@vueuse/core';
import { nanoid } from '@0x-jerry/utils'

interface AIConfig {
  id: string
  label: string
  baseUrl: string
  apiKey: string
}
const configs = useLocalStorage<AIConfig[]>('ai-config', [])

async function save() {
  // todo
}

async function add() {
  configs.value.push({
    id: nanoid(),
    label: '',
    baseUrl: '',
    apiKey: ''
  })
}

async function remove(idx: number) {
  configs.value.splice(idx, 1)
}
</script>

<template>
  <div>
    <div v-for="(conf, idx) in configs" :key="conf.id">
      <div>
        <label>
          Label
          <button @click="remove(idx)">Remove</button>
        </label>
        <input v-model="conf.label" />
      </div>
      <div>
        <label>baseUrl</label>
        <input v-model="conf.baseUrl" />
      </div>
      <div>
        <label>API Key</label>
        <input v-model="conf.apiKey" />
      </div>
    </div>
    <div>
      <button @click="add">Add</button>
    </div>
    <div>
      <button @click="save">Save</button>
    </div>
  </div>
</template>

<style lang='less' scoped></style>