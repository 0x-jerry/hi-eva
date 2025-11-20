<script lang='ts' setup>
import type { IFuseOptions } from 'fuse.js'
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import { computed } from 'vue'
import {
  BuiltinConfigItem,
  BuiltinEndpointsConfig,
} from '../../logic/builtinConfig'
import { getEndpointConf } from '../../logic/config'
import AutoCompleteInput from '../AutoCompleteInput.vue'
import Icon from '../Icon.vue'

export interface EndpointSettingProps {
  confId: string
}

const emit = defineEmits(['remove'])

const props = defineProps<EndpointSettingProps>()

const conf = computed(() => getEndpointConf(props.confId))

const fuseOption: IFuseOptions<BuiltinConfigItem> = {
  includeScore: true,
  keys: ['baseUrl', 'name'],
}
</script>

<template>

  <div class="endpoint-setting" v-if="conf">
    <div class="flex flex-col gap-2 bg-light-3 p-4 rounded" :key="conf.id">
      <div class="flex items-center">
        <Inplace :disabled="conf.builtin">
          <template #display>
            {{ conf.label || '无配置名称' }}
          </template>
          <template #content="{ closeCallback }">
            <span class="inline-flex items-center gap-2">
              <InputText v-model="conf.label" autofocus />
              <Icon class="i-carbon:checkmark cursor-pointer" @click="closeCallback" />
            </span>
          </template>
        </Inplace>
        <div class="flex flex-1 justify-end">
          <Icon v-if="!conf.builtin" class="i-carbon:close cursor-pointer" @click="emit('remove')" />
        </div>
      </div>
      <div class="editable-row">
        <label>Base URL</label>
        <AutoCompleteInput class="content" v-model="conf.baseUrl" :items="BuiltinEndpointsConfig" optionLabel="baseUrl" :fuse="fuseOption" />
      </div>
      <div class="editable-row">
        <label>API Key</label>
        <Password class="content" v-model="conf.apiKey" toggleMask :feedback="false"/>
      </div>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.editable-row {
  @apply flex items-center;

  label {
    width: 100px;
    text-align: right;
    margin-right: 0.5rem;
  }

  .content {
    width: calc(100% - 150px);

    &:deep(.p-inputtext) {
      width: 100%;
    }
  }

}
</style>
