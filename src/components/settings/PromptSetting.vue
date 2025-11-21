<script lang='ts' setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { Button } from 'primevue'
import { ref } from 'vue'
import { endpointConfigTable } from '../../database/endpointConfig'
import type { IPromptConfigItem } from '../../database/promptConfig'
import { promptConfigTable } from '../../database/promptConfig'
import Icon from '../Icon.vue'
import PromptItemSetting from './PromptItemSetting.vue'
import SettingTitle from './SettingTitle.vue'

const configsApi = useAsyncData(() => promptConfigTable.findAll(), [])

configsApi.load()

const newData = ref<Partial<IPromptConfigItem>>()

const endpointsApi = useAsyncData(() => endpointConfigTable.findAll(), [])

endpointsApi.load()

function addConfig() {
  newData.value = {
    name: '未命名',
    icon: '',
    prompt: '',
  }
}

function resetNewData() {
  newData.value = undefined
}

async function removeConf(conf: IPromptConfigItem) {
  if (conf.id) {
    await promptConfigTable.deleteById(conf.id)
  }

  await configsApi.load()
}

async function handleAddOrUpdate(conf: IPromptConfigItem) {
  if (conf.id) {
    await promptConfigTable.updateOne({ ...conf, id: conf.id })
  } else {
    await promptConfigTable.createOne(conf)
    newData.value = undefined
  }

  await configsApi.load()
}

function updateEndpoints() {
  endpointsApi.load()
}

defineExpose({
  updateEndpoints,
})
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Prompt 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <PromptItemSetting v-if="newData" :item="newData" :endpoints="endpointsApi.data.value" @update="handleAddOrUpdate"
        @remove="resetNewData" />

      <template v-if="!newData && !configsApi.data.value.length">
        <div class="flex text-center justify-center py-8 bg-light-2">
          <Button variant="text" @click="addConfig">+ 新增配置</Button>
        </div>
      </template>
      <template v-else>
        <PromptItemSetting v-for="conf in configsApi.data.value" :key="conf.id" :item="conf"
          :endpoints="endpointsApi.data.value" @remove="removeConf(conf)" @update="handleAddOrUpdate" />
      </template>
    </div>
  </div>
</template>

<style lang='scss' scoped></style>
