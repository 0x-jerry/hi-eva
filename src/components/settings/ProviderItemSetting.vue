<script lang="ts" setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import OpenAI from 'openai'
import {
  Button,
  Form,
  FormInstanceFunctions,
  FormItem,
  Input,
  Select,
} from 'tdesign-vue-next'
import { useTemplateRef } from 'vue'
import { IEndpointConfigItem } from '../../database/endpointConfig'

export interface EndpointItemSettingProps {
  _?: unknown
}

const props = defineProps<EndpointItemSettingProps>()

const value = defineModel<IEndpointConfigItem>({ required: true })

const formRef = useTemplateRef<FormInstanceFunctions>('formRef')

const modelListApi = useAsyncData(refreshModelList, [])

async function refreshModelList() {
  // todo
  const ins = new OpenAI({
    dangerouslyAllowBrowser: true,
    baseURL: '',
    apiKey: '',
  })

  const list = await ins.models.list()

  return list.data.map((n) => ({
    label: n.id,
    value: n.id,
  }))
}

function validate() {
  formRef.value?.validate()
}

defineExpose({
  validate,
})
</script>

<template>
  <div class="endpoint-item-setting">
    <Form ref="formRef" :data="value" label-align="top">
      <FormItem label=" Name" name="name">
        <Input v-model="value.name"></Input>
      </FormItem>
      <FormItem label="BaseURL" name="baseUrl">
        <Input v-model="value.baseUrl"></Input>
      </FormItem>
      <FormItem label="API Key" name="apiKey">
        <Input v-model="value.apiKey" type="password"></Input>
      </FormItem>
      <FormItem label="Model" name="model">
        <Select v-model="value.model" :options="modelListApi.data.value"></Select>
        <template #panelBottomContent>
          <Button block @click="modelListApi.load()">Refresh List</Button>
        </template>
      </FormItem>
    </Form>
  </div>
</template>

<style lang="scss" scoped>
</style>
