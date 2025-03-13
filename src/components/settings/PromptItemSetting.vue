<script lang='ts' setup>
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import Textarea from 'primevue/textarea'
import { aiEndPointConfigs, getPromptConf } from '../../logic/config'
import Icon from '../Icon.vue'
import IconPicker from '../IconPicker.vue'
import { computed } from 'vue'

export interface PromptItemSettingProps {
  confId: string
}

const emit = defineEmits(['remove'])

const props = defineProps<PromptItemSettingProps>()

const conf = computed(() => getPromptConf(props.confId))

const modelList = computed(
  () =>
    aiEndPointConfigs.value.find((c) => c.id === conf.value?.enpointId)
      ?.models || [],
)
</script>

<template>
  <div class="flex flex-col gap-2 bg-light-3 p-4 rounded" v-if="conf">
    <div class="flex items-center gap-1">
      <IconPicker v-model="conf.icon" />
      <Inplace :disabled="conf.builtin">
        <template #display>
          {{ conf.name || '未命名' }}
        </template>
        <template #content="{ closeCallback }">
          <span class="inline-flex items-center gap-2">
            <InputText v-model="conf.name" autofocus />
            <Icon class="i-carbon:checkmark cursor-pointer" @click="closeCallback" />
          </span>
        </template>
      </Inplace>
      <div class="flex flex-1 justify-end">
        <Icon v-if="!conf.builtin" class="i-carbon:close cursor-pointer" @click="emit('remove')" />
      </div>
    </div>
    <div class="flex flex-col gap-2">
      <Textarea v-model="conf.prompt" :rows="5" />
    </div>
    <div class="editable-row">
      <label>Models</label>
      <div class="flex gap-2">
        <Select v-model="conf.enpointId" :options="aiEndPointConfigs" optionLabel="label" optionValue="id"
          placeholder="Select a endpoint" />

        <Select v-model="conf.model" :options="modelList" placeholder="Select a model" />
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

  &:deep(.p-inputtext) {
    width: calc(100% - 150px);
  }
}
</style>
