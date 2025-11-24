<script lang='ts' setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { Button } from 'tdesign-vue-next'
import { ref } from 'vue'
import {
  endpointConfigTable,
  IEndpointConfigItem,
} from '../../database/endpointConfig'
import Icon from '../Icon.vue'
import ProviderItemSetting from './ProviderItemSetting.vue'
import SettingTitle from './SettingTitle.vue'

export type EndpointSettingEmit = {
  updated: []
}

const emit = defineEmits<EndpointSettingEmit>()

const configsApi = useAsyncData(async () => {
  const resp = await endpointConfigTable.findAll()

  return resp
}, [])

configsApi.load()

const newData = ref<IEndpointConfigItem>()

function addConfig() {
  newData.value = {
    apiKey: '',
    name: '未命名',
    baseUrl: '',
    model: '',
  }
}

function resetNewData() {
  newData.value = undefined
}

async function removeConf(conf: IEndpointConfigItem) {
  if (conf.id) {
    await endpointConfigTable.deleteById(conf.id)
  }

  await configsApi.load()
  emit('updated')
}

async function handleAddOrUpdate(conf: IEndpointConfigItem) {
  if (conf.id) {
    await endpointConfigTable.updateOne({
      ...conf,
      id: conf.id,
    })
  } else {
    await endpointConfigTable.createOne(conf)
    newData.value = undefined
  }

  await configsApi.load()
  emit('updated')
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Provider 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <ProviderItemSetting v-if="newData" :item="newData" @update="handleAddOrUpdate" @remove="resetNewData" />

      <template v-if="!newData && !configsApi.data.value.length">
          <div class="flex text-center justify-center py-8 bg-light-2">
            <Button variant="text" @click="addConfig" >
              + 新增配置
            </Button>
          </div>
      </template>
      <template v-else>
        <ProviderItemSetting v-for="conf in configsApi.data.value" :key="conf.id" :item="conf" @remove="removeConf(conf)" @update="handleAddOrUpdate" />
      </template>
    </div>
  </div>
</template>

<style lang='scss' scoped></style>
