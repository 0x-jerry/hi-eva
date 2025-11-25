<script lang='ts' setup>
import { sleep } from '@0x-jerry/utils'
import { Button, Input, MessagePlugin, Select, Switch } from 'tdesign-vue-next'
import { useBasicConfig, useI18n } from '../../composables'
import { commands } from '../../logic/commands'
import { applyLangUpdate, availableLangs, defaultLang } from '../../logic/i18n'
import ShortcutInput from '../ShortcutInput.vue'
import SettingTitle from './SettingTitle.vue'

const basicConfig = useBasicConfig()
const { t } = useI18n()

const langOptions = availableLangs.map((item) => ({
  label: item,
  value: item,
}))

async function applyClipboardChange() {
  await sleep(100)
  await commands.applyClipboardListener()
}

function openSettingFolder() {
  commands.openSettingFolder()
}

function updateLang() {
  applyLangUpdate()
}

async function applyGlobalShortcut() {
  MessagePlugin.success('changed')
  await sleep(100)
  await commands.applyGlobalShortcut()
}

async function applyAutoTrigger() {
  await sleep(100)
  await commands.applyAutoTrigger()
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        {{ t('main.basicSettings') }}
      </span>
      <div class="flex-1 flex justify-end">
        <Button variant="text" @click="openSettingFolder">{{ t('basicsetting.openSettingFolder') }}</Button>
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <div class="field-row">
        <label>{{ t('basicsetting.lang') }}</label>
        <Select class="flex-1" v-model="defaultLang" :options="langOptions" @change="updateLang" />
      </div>
      <div class="field-row">
        <label>{{ t('basicsetting.proxy') }}</label>
        <Input class="flex-1" v-model="basicConfig.proxy" :placeholder="t('basicsetting.proxy')" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.listenClipboard') }}</label>
        <Switch  v-model="basicConfig.enableListenClipboard" @change="applyClipboardChange" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.autoTrigger') }}</label>
        <Switch  v-model="basicConfig.enableAutoTrigger" @change="applyAutoTrigger" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.shortcut') }}</label>
        <div class="flex gap-1 items-center w-full">
          <Switch v-model="basicConfig.enableGlobalShortcut" @change="applyGlobalShortcut" />
          <ShortcutInput v-model="basicConfig.globalShortcut" @blur="applyGlobalShortcut" />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.field-row {
  --uno: flex items-center gap-2;

  label {
    width: 6em;
    text-align: right;
  }
}
</style>
