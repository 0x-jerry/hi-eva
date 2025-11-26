<script lang='ts' setup>
import { Button, Input, MessagePlugin, Select, Switch } from 'tdesign-vue-next'
import { useAppBasicConfig, useI18n } from '../../composables'
import { commands } from '../../logic/commands'
import { applyLangUpdate, availableLangs, defaultLang } from '../../logic/i18n'
import ShortcutInput from '../ShortcutInput.vue'
import SettingTitle from './SettingTitle.vue'

const basicConfig = useAppBasicConfig()
const { t } = useI18n()

basicConfig.load()

const langOptions = availableLangs.map((item) => ({
  label: item,
  value: item,
}))

async function applyClipboardChange() {
  await basicConfig.save()
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
  await basicConfig.save()
  await commands.applyGlobalShortcut()
}

async function applyAutoTrigger() {
  await basicConfig.save()
  await commands.applyAutoTrigger()
}

async function saveConfig() {
  await basicConfig.save()
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
        <Input class="flex-1" v-model="basicConfig.data.proxy" :placeholder="t('basicsetting.proxy')" @blur="saveConfig" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.listenClipboard') }}</label>
        <Switch  v-model="basicConfig.data.enableListenClipboard" @change="applyClipboardChange" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.autoTrigger') }}</label>
        <Switch  v-model="basicConfig.data.enableAutoTrigger" @change="applyAutoTrigger" />
      </div>
      <div class="field-row ">
        <label>{{ t('basicsetting.shortcut') }}</label>
        <div class="flex gap-1 items-center w-full">
          <Switch v-model="basicConfig.data.enableGlobalShortcut" @change="applyGlobalShortcut" />
          <ShortcutInput v-model="basicConfig.data.globalShortcut" @blur="applyGlobalShortcut" />
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
