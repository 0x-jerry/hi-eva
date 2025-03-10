<script lang='ts' setup>
import { nanoid } from '@0x-jerry/utils'
import SettingTitle from './SettingTitle.vue'
import InputText from 'primevue/inputtext'
import Icon from '../Icon.vue'
import Inplace from 'primevue/inplace'
import {
	aiEndPointConfigs,
	promptConfigs,
	type ToolbarPromptConfig,
} from '../../logic/config'
import Select from 'primevue/select'

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
        <div class="flex items-center">
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
          <div class="flex flex-1 justify-end">
            <Icon v-if="!conf.builtin" class="i-carbon:close" @click="removeConf(idx)" />
          </div>
        </div>
        <div class="editable-row">
          <!-- create a icon picker component -->
          <label>icon</label>
          <InputText v-model="conf.icon" />
        </div>
        <div class="editable-row">
          <label>prompt</label>
          <InputText v-model="conf.prompt" />
        </div>
        <div class="editable-row">
          <label>Models</label>
          <div>

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