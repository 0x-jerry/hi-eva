<script lang='ts' setup>
import { useLocalStorage } from '@vueuse/core';
import { nanoid } from '@0x-jerry/utils'
import SettingTitle from './SettingTitle.vue';
import InputText from 'primevue/inputtext';
import Icon from '../Icon.vue';
import Inplace from 'primevue/inplace';


interface AIConfig {
  id: string
  builtin?: boolean
  label: string
  baseUrl: string
  apiKey: string
}

const builtinConfigs: AIConfig[] = [{
  id: 'dashscope',
  builtin: true,
  label: '百炼',
  baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
  apiKey: ''
}]

const configs = useLocalStorage<AIConfig[]>('ai-config', builtinConfigs)

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
    <SettingTitle class="mb-2 gap-2">
      <span>
        AI 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add" @click="add" />
      </div>
    </SettingTitle>


    <div class="flex flex-col gap-2">
      <div class="flex flex-col gap-2 bg-light-3 p-4 rounded" v-for="(conf, idx) in configs" :key="conf.id">
        <div class="flex items-center">
          <Inplace :disabled="conf.builtin">
            <template #display>
              {{ conf.label }}
            </template>
            <template #content="{ closeCallback }">
              <span class="inline-flex items-center gap-2">
                <InputText v-model="conf.label" autofocus />
                <Icon class="i-carbon:checkmark" @click="closeCallback" />
              </span>
            </template>
          </Inplace>
          <div class="flex flex-1 justify-end">
            <Icon v-if="!conf.builtin" class="i-carbon:close" @click="remove(idx)" />
          </div>
        </div>
        <div class="editable-row">
          <label>baseUrl</label>
          <InputText v-model="conf.baseUrl" />
        </div>
        <div class="editable-row">
          <label>API Key</label>
          <InputText v-model="conf.apiKey" />
        </div>
      </div>
    </div>

  </div>
</template>

<style lang='less' scoped>
.editable-row {
  @apply flex items-center;

  label {
    width: 100px;
    text-align: right;
    margin-right: 0.5rem;
  }

  &:deep(.p-inputtext) {
    width: calc(100% - 150px);
  }
}
</style>