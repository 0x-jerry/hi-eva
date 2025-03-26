<script lang='ts' setup>
import SettingTitle from './SettingTitle.vue'
import { useBasicConfig } from '../../composables'
import { InputText, Checkbox } from 'primevue'
import { useId, watch } from 'vue'

const basicConfig = useBasicConfig()

const listenClipboardFieldId = useId()

watch(
  () => basicConfig.value.listenClipboard,
  (val) => {
    console.log(val)
  },
)
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        基础配置
      </span>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <div class="field-row">
        <label> 代理地址</label>
        <InputText class="flex-1" v-model="basicConfig.proxy" placeholder="代理地址" />
      </div>
      <div class="field-row ">
        <label :for="listenClipboardFieldId"> 监听剪贴板 </label>
        <Checkbox :input-id="listenClipboardFieldId" v-model="basicConfig.listenClipboard" binary>  </Checkbox>
      </div>
    </div>
  </div>
</template>

<style lang='less' scoped>
.field-row {
  @apply flex items-center gap-2;

  label {
    width: 6em;
    text-align: right;
  }
}
</style>
