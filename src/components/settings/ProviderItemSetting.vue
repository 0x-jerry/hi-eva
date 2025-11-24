<script lang="ts" setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { watchImmediate } from '@vueuse/core'
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
import { fetchModelList } from '../../logic/modelListUtils'

export interface EndpointItemSettingProps {
  _?: unknown
}

const props = defineProps<EndpointItemSettingProps>()

const value = defineModel<IEndpointConfigItem>({ required: true })

const formRef = useTemplateRef<FormInstanceFunctions>('formRef')

const modelListApi = useAsyncData(updateModelList, [])

watchImmediate(value, () => {
  modelListApi.load()
})

async function updateModelList(force = false) {
  const { baseUrl, apiKey } = value.value

  if (!baseUrl) {
    return []
  }

  if (force) {
    return fetchModelList.forceUpdate({
      baseUrl,
      apiKey,
    })
  }

  return fetchModelList({
    baseUrl,
    apiKey,
  })
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
        <Select v-model="value.model" :options="modelListApi.data.value" :loading="modelListApi.isLoading.value" filterable>
          <template #panelBottomContent>
            <Button block variant="text" theme="primary" @click="modelListApi.load(true)">Refresh List</Button>
          </template>
        </Select>
      </FormItem>
    </Form>
  </div>
</template>

<style lang="scss" scoped>
</style>
