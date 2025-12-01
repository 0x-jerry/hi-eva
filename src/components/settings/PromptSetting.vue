<script lang="ts" setup>
import { useAsyncData } from '@0x-jerry/vue-kit'
import { Button, List, ListItem } from 'tdesign-vue-next'
import { ref, toRaw } from 'vue'
import { VueDraggable } from 'vue-draggable-plus'
import { useDrawer, useI18n } from '../../composables'
import type { IPromptConfigItem } from '../../database/promptConfig'
import { promptConfigTable } from '../../database/promptConfig'
import CarbonIcon from '../CarbonIcon.vue'
import Icon from '../Icon.vue'
import PromptItemSetting from './PromptItemSetting.vue'
import SettingTitle from './SettingTitle.vue'

const editDrawer = useDrawer()
const { t } = useI18n()

const configsApi = useAsyncData(promptConfigTable.getAllSorted, [])

configsApi.load()

const editData = ref<IPromptConfigItem>({
  name: t('common.untitled'),
  icon: '',
  prompt: '',
})

function resetEditData() {
  editData.value = {
    name: t('common.untitled'),
    icon: '',
    prompt: '',
  }
}

async function removeConf(conf: IPromptConfigItem) {
  if (conf.id) {
    await promptConfigTable.deleteById(conf.id)
  }

  await configsApi.load()
}

async function handleAddOrUpdate() {
  const conf = editData.value
  conf.orderNo = conf.orderNo ?? configsApi.data.value.length

  if (conf.id) {
    await promptConfigTable.updateOne({ ...conf, id: conf.id })
  } else {
    await promptConfigTable.createOne(conf)
  }

  await configsApi.load()

  editDrawer.close()
  resetEditData()
}

function openEditDrawer(conf: IPromptConfigItem) {
  editData.value = structuredClone(toRaw(conf))

  editDrawer.open()
}

function handleNewFunction() {
  resetEditData()

  editDrawer.open()
}

async function applyChangeOrder() {
  const data = [...configsApi.data.value]
  data.forEach((item, i) => {
    item.orderNo = i
  })

  await promptConfigTable.batchSaveOrder(data)
  await configsApi.load()
}
</script>

<template>
  <div>
    <SettingTitle class="mb-2 gap-2">
      <span>{{ t('promptsetting.title') }}</span>
      <div class="flex items-center gap-2">
        <Icon class="i-carbon:add cursor-pointer" @click="handleNewFunction" />
      </div>
    </SettingTitle>

    <div class="flex flex-col gap-2">
      <template v-if="!configsApi.data.value.length">
        <div class="flex text-center justify-center py-8 bg-light-2">
          <Button variant="text" @click="handleNewFunction">+ {{ t('promptsetting.addConfig') }}</Button>
        </div>
      </template>
      <template v-else>
        <List>
          <VueDraggable v-model="configsApi.data.value" handle=".handle" :animation="200" @end="applyChangeOrder">
            <ListItem v-for="conf in configsApi.data.value" :key="conf.id">
              <Icon class="i-carbon:draggable handle cursor-move !px-2" />
              <div class="gap-1 flex flex-1 items-center">
                <div v-if="conf.icon"
                  class="text-xl border-(1 solid light-8) rounded size-6 flex items-center justify-center">
                  <CarbonIcon :name="conf.icon" />
                </div>
                <span>
                  {{ conf.name }}
                </span>
              </div>

              <template #action>
                <Icon class="i-carbon:edit cursor-pointer" @click="openEditDrawer(conf)" />
                <Icon class="i-carbon:trash-can cursor-pointer" @click="removeConf(conf)" />
              </template>
            </ListItem>

          </VueDraggable>
        </List>
      </template>
    </div>

    <editDrawer.Component :header="t('promptsetting.editPrompt')" @confirm="handleAddOrUpdate">
      <PromptItemSetting v-model="editData" />
    </editDrawer.Component>
  </div>
</template>

<style lang="scss" scoped></style>
