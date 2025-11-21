<script lang='ts' setup>
import { Button, Checkbox, InputText } from 'primevue'
import { useId } from 'vue'
import { useBasicConfig } from '../../composables'
import { commands } from '../../logic/commands'
import SettingTitle from './SettingTitle.vue'

const basicConfig = useBasicConfig()

const listenClipboardFieldId = useId()

async function applyClipboardChange() {
  await commands.applyClipboardListener()
}

function openSettingFolder() {
  commands.openSettingFolder()
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        基础配置
      </span>
      <div class="flex-1 flex justify-end">
        <Button variant="text" @click="openSettingFolder">打开配置文件夹</Button>
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <div class="field-row">
        <label> 代理地址</label>
        <InputText class="flex-1" v-model="basicConfig.proxy" placeholder="代理地址" />
      </div>
      <div class="field-row ">
        <label :for="listenClipboardFieldId"> 监听剪贴板 </label>
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
