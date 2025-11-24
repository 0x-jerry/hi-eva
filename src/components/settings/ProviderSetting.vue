<script lang='ts' setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { Button, List, ListItem, Tag } from 'tdesign-vue-next'
import { ref, toRaw } from 'vue'
import { useDrawer } from '../../composables/useDrawer'
import {
  endpointConfigTable,
  IEndpointConfigItem,
} from '../../database/endpointConfig'
import Icon from '../Icon.vue'
import ProviderItemSetting from './ProviderItemSetting.vue'
import SettingTitle from './SettingTitle.vue'

export type EndpointSettingEmit = {
  updated: []
}

const emit = defineEmits<EndpointSettingEmit>()

const editDrawer = useDrawer()

const configsApi = useAsyncData(async () => {
  const resp = await endpointConfigTable.findAll()

  return resp
}, [])

configsApi.load()

const editData = ref<IEndpointConfigItem>({
  apiKey: '',
  name: '未命名',
  baseUrl: '',
  model: '',
})

function resetEditData() {
  editData.value = {
    apiKey: '',
    name: '未命名',
    baseUrl: '',
    model: '',
  }
}

async function removeConf(conf: IEndpointConfigItem) {
  if (conf.id) {
    await endpointConfigTable.deleteById(conf.id)
  }

  await configsApi.load()
  emit('updated')
}

async function handleAddOrUpdate() {
  const conf = editData.value

  if (conf.id) {
    await endpointConfigTable.updateOne({
      ...conf,
      id: conf.id,
    })
  } else {
    await endpointConfigTable.createOne(conf)
    resetEditData()
  }

  await configsApi.load()
  editDrawer.close()
  emit('updated')
}

function openEditDrawer(conf: IEndpointConfigItem) {
  editData.value = structuredClone(toRaw(conf))

  editDrawer.open()
}

function handleNewFunction() {
  resetEditData()

  editDrawer.open()
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>
        Provider 配置
      </span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="handleNewFunction" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <template v-if="!configsApi.data.value.length">
        <div class="flex text-center justify-center py-8 bg-light-2">
          <Button variant="text" @click="handleNewFunction">
            + 新增配置
          </Button>
        </div>
      </template>
      <template v-else>
        <List>
          <ListItem v-for="conf in configsApi.data.value">
            <div class="flex items-center gap-1">
              <span>{{ conf.name }}</span>
              <Tag v-if="conf.model" size="small">{{ conf.model }}</Tag>
            </div>

            <template #action>
              <Icon class="i-carbon:edit cursor-pointer" @click="openEditDrawer(conf)" />
              <Icon class="i-carbon:trash-can cursor-pointer" @click="removeConf(conf)" />
            </template>
          </ListItem>
        </List>
      </template>
    </div>

    <editDrawer.Component header="Edit Provider" @confirm="handleAddOrUpdate">
      <ProviderItemSetting v-model="editData" />
    </editDrawer.Component>
  </div>
</template>

<style lang='scss' scoped></style>
