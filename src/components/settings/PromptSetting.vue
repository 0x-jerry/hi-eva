<script lang='ts' setup>
import { nanoid } from '@0x-jerry/utils'
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import Textarea from 'primevue/textarea'
import {
  type ToolbarPromptConfig,
  aiEndPointConfigs,
  promptConfigs,
} from '../../logic/config'
import Icon from '../Icon.vue'
import SettingTitle from './SettingTitle.vue'
import IconPicker from '../IconPicker.vue'

const configs = promptConfigs

async function addConfig() {
  configs.value.push({
    id: nanoid(),
    name: '',
    icon: '',
    prompt: '',
  })
}

async function removeConf(idx: number) {
  configs.value.splice(idx, 1)
}

function getSelectedEndpointModelList(conf: ToolbarPromptConfig) {
  return (
    aiEndPointConfigs.value.find((c) => c.id === conf.enpointId)?.models || []
  )
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Prompt 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <div class="flex flex-col gap-2 bg-light-3 p-4 rounded" v-for="(conf, idx) in configs" :key="conf.id">
        <div class="flex items-center gap-2">
          <Inplace :disabled="conf.builtin">
            <template #display>
              {{ conf.name }}
            </template>
            <template #content="{ closeCallback }">
              <span class="inline-flex items-center gap-2">
                <InputText v-model="conf.name" autofocus />
                <Icon class="i-carbon:checkmark" @click="closeCallback" />
              </span>
            </template>
          </Inplace>
          <IconPicker v-model="conf.icon" />
          <div class="flex flex-1 justify-end">
            <Icon v-if="!conf.builtin" class="i-carbon:close" @click="removeConf(idx)" />
          </div>
        </div>
        <div class="flex flex-col gap-2">
          <label>prompt</label>
          <Textarea v-model="conf.prompt" :rows="5" />
        </div>
        <div class="editable-row">
          <label>Models</label>
          <div class="flex gap-2">
            <Select v-model="conf.enpointId" :options="aiEndPointConfigs" optionLabel="label" optionValue="id"
              placeholder="Select a endpoint" />

            <Select v-model="conf.model" :options="getSelectedEndpointModelList(conf)" placeholder="Select a model" />
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

  &:deep(.p-inputtext) {
    width: calc(100% - 150px);
  }
}
</style>
