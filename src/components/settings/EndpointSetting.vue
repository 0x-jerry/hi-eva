<script lang='ts' setup>
import { parseJson } from '@0x-jerry/utils'
import { useAsyncData } from '@0x-jerry/vue-kit'
import { ref } from 'vue'
import { endpointConfigTable } from '../../database/endpointConfig'
import { EndpointItem } from '../../types/endpoint'
import Icon from '../Icon.vue'
import EndpointItemSetting from './EndpointItemSetting.vue'
import SettingTitle from './SettingTitle.vue'


const configs = useAsyncData(async () => {
  const resp = await endpointConfigTable.findAll()

  return resp.map(item => ({ ...item, models: parseJson<string[]>(item.models) }))
}, [])

const newData = ref<EndpointItem>()

async function addConfig() {
  newData.value = {
    apiKey: '',
    models: [],
    name: '',
    baseUrl: ''
  }
}

async function removeConf(idx: number) {
  // configs.value.splice(idx, 1)
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Endpoint 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <EndpointItemSetting v-if="newData" v-model="newData" />

      <!-- todo -->
      <!-- <EndpointSetting v-for="(conf, idx) in configs" :confId="conf.id" :key="conf.id" @remove="removeConf(idx)" /> -->
    </div>
  </div>
</template>

<style lang='scss' scoped></style>
