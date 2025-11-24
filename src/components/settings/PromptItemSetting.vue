<script lang='ts' setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { watchImmediate } from '@vueuse/core'
import {
  Form,
  FormInstanceFunctions,
  FormItem,
  Input,
  Select,
  Textarea,
} from 'tdesign-vue-next'
import { useTemplateRef } from 'vue'
import { endpointConfigTable } from '../../database/endpointConfig'
import type { IPromptConfigItem } from '../../database/promptConfig'
import IconPicker from '../IconPicker.vue'

export interface PromptItemSettingProps {
  _?: unknown
}

const props = defineProps<PromptItemSettingProps>()

const value = defineModel<IPromptConfigItem>({ required: true })

const formRef = useTemplateRef<FormInstanceFunctions>('formRef')

const endpointsApi = useAsyncData(endpointConfigTable.findAll, [])

watchImmediate(value, () => {
  endpointsApi.load()
})

function validate() {
  formRef.value?.validate()
}

defineExpose({
  validate,
})
</script>

<template>
  <div class="prompt-item-setting">
    <Form ref="formRef" :data="value" label-align="top">
      <FormItem label=" Name" name="name">
        <div class="flex gap-1 w-full">
          <IconPicker v-model="value.icon" />
          <Input class="flex-1" v-model="value.name"></Input>
        </div>
      </FormItem>
      <FormItem label="Provider" name="endpointId">
        <Select v-model="value.endpointId" :options="endpointsApi.data.value"
          :keys="{ label: 'name', value: 'id' }"></Select>
      </FormItem>
      <FormItem label="Prompt" name="prompt">
        <Textarea v-model="value.prompt" placeholder="Write a prompt..."
          :autosize="{ minRows: 3, maxRows: 5 }"></Textarea>
      </FormItem>
    </Form>
  </div>
</template>

<style lang='scss' scoped></style>
