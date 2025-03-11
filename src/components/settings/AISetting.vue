<script lang='ts' setup>
import { nanoid } from '@0x-jerry/utils'
import { remove, uniq } from 'lodash-es'
import Chip from 'primevue/chip'
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import { type AIEndpointConfig, aiEndPointConfigs } from '../../logic/config'
import Icon from '../Icon.vue'
import SettingTitle from './SettingTitle.vue'

const configs = aiEndPointConfigs

async function add() {
  configs.value.push({
    id: nanoid(),
    label: '',
    baseUrl: '',
    apiKey: '',
    models: [],
  })
}

async function removeConf(idx: number) {
  configs.value.splice(idx, 1)
}

function handleAddModel(event: KeyboardEvent, conf: AIEndpointConfig) {
  if (event.code !== 'Enter') {
    return
  }

  const el = event.target as HTMLInputElement
  const modelName = el.value.trim()
  if (!modelName) {
    return
  }

  const models = [...conf.models, modelName]

  conf.models = uniq(models)
}

function removeModel(conf: AIEndpointConfig, model: string) {
  remove(conf.models, (v) => v === model)
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
            <Icon v-if="!conf.builtin" class="i-carbon:close" @click="removeConf(idx)" />
          </div>
        </div>
        <div class="editable-row">
          <label>baseUrl</label>
          <InputText class="content" v-model="conf.baseUrl" />
        </div>
        <div class="editable-row">
          <label>API Key</label>
          <Password class="content" v-model="conf.apiKey" toggleMask />
        </div>
        <div class="editable-row">
          <label>Models</label>
          <div class="flex flex-wrap gap-2 flex-1">
            <Chip v-for="model in conf.models" :label="model" removable @remove="removeModel(conf, model)" />
            <InputText @keydown="handleAddModel($event, conf)" />
          </div>
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

  .content {
    width: calc(100% - 150px);

    &:deep(.p-inputtext) {
      width: 100%;
    }
  }

}
</style>
