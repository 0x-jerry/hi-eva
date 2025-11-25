<script lang='ts' setup>
import { Button, Checkbox, Input, Select } from 'tdesign-vue-next'
import { useId } from 'vue'
import { useBasicConfig, useI18n } from '../../composables'
import { commands } from '../../logic/commands'
import { applyLangUpdate, availableLangs, defaultLang } from '../../logic/i18n'
import SettingTitle from './SettingTitle.vue'

const basicConfig = useBasicConfig()
const { t } = useI18n()

const langOptions = availableLangs.map((item) => ({
  label: item,
  value: item,
}))

const listenClipboardFieldId = useId()

async function applyClipboardChange() {
  await commands.applyClipboardListener()
}

function openSettingFolder() {
  commands.openSettingFolder()
}

function updateLang() {
  applyLangUpdate()
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
        <label :for="listenClipboardFieldId">{{ t('basicsetting.listenClipboard') }}</label>
        <Checkbox :input-id="listenClipboardFieldId" v-model="basicConfig.listenClipboard" binary @change="applyClipboardChange">  </Checkbox>
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
