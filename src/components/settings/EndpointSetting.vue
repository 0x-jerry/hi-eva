<script lang='ts' setup>
import Chip from 'primevue/chip'
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import { computed, reactive } from 'vue'
import { getEndpointConf } from '../../logic/config'
import { uniq } from 'lodash-es'
import Icon from '../Icon.vue'

export interface EndpointSettingProps {
  confId: string
}

const emit = defineEmits(['remove'])

const props = defineProps<EndpointSettingProps>()

const conf = computed(() => getEndpointConf(props.confId))

const state = reactive({
  model: '',
})

function removeModel(model: string) {
  if (!conf.value) return
  conf.value.models = conf.value.models.filter((m: string) => m !== model)
}

function handleAddModel(event: KeyboardEvent) {
  if (!conf.value) return

  if (event.code !== 'Enter') {
    return
  }

  const modelName = state.model.trim()

  if (!modelName) {
    return
  }

  const models = [...conf.value.models, modelName]

  conf.value.models = uniq(models)
  state.model = ''
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
        <InputText class="content" v-model="conf.baseUrl" />
      </div>
      <div class="editable-row">
        <label>API Key</label>
        <Password class="content" v-model="conf.apiKey" toggleMask :feedback="false"/>
      </div>
      <div class="editable-row">
        <label>Model List</label>
        <div class="flex flex-wrap gap-2 flex-1">
          <Chip v-for="model in conf.models" :label="model" removable @remove="removeModel(model)" />
          <InputText v-model="state.model" @keydown="handleAddModel($event)" />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang='less' scoped>
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
