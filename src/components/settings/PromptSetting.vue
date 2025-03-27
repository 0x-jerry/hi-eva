<script lang='ts' setup>
import { nanoid } from '@0x-jerry/utils'
import { promptConfigs } from '../../logic/config'
import Icon from '../Icon.vue'
import SettingTitle from './SettingTitle.vue'
import PromptItemSetting from './PromptItemSetting.vue'

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
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Prompt 配置
      </span>
      <div class="flex items-center gap-2 cursor-pointer">
        <Icon class="i-carbon:add" @click="addConfig" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <PromptItemSetting v-for="(conf, idx) in configs" :confId="conf.id" :key="conf.id" @remove="removeConf(idx)" />
    </div>
  </div>
</template>

<style lang='less' scoped>
</style>
