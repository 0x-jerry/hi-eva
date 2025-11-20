<script lang='ts' setup>
import { nanoid } from '@0x-jerry/utils'
import { endPointConfigs } from '../../logic/config'
import Icon from '../Icon.vue'
import EndpointSetting from './EndpointItemSetting.vue'
import SettingTitle from './SettingTitle.vue'

const configs = endPointConfigs

async function addConfig() {
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
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        AI 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <EndpointSetting v-for="(conf, idx) in configs" :confId="conf.id" :key="conf.id" @remove="removeConf(idx)" />
    </div>
  </div>
</template>

<style lang='scss' scoped>
</style>
