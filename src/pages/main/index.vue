<script setup lang="ts">
import { useAsyncData } from '@0x-jerry/vue-kit'
import dayjs from 'dayjs'
import { Button, Empty, Input, Select } from 'tdesign-vue-next'
import { computed, reactive } from 'vue'
import ChatWithHistory from '../../components/ChatWithHistory.vue'
import ClickToEdit from '../../components/ClickToEdit.vue'
import Icon from '../../components/Icon.vue'
import { useI18n } from '../../composables'
import { chatHistoryTable, IChatHistoryModel } from '../../database/chatHistory'
import { chatHistoryMsgTable } from '../../database/chatHistoryMsg'
import { endpointConfigTable } from '../../database/endpointConfig'

const historiesApi = useAsyncData(chatHistoryTable.findAll, [])

const historyMessageApi = useAsyncData(chatHistoryMsgTable.getMsgs, [])

const endpointConfigsApi = useAsyncData(endpointConfigTable.findAll, [])

const { t } = useI18n()

const state = reactive({
  selectedId: null as number | null,
  endpointConfigId: undefined as number | undefined,
})

const selectedHistory = computed(() => {
  return historiesApi.data.value.find((h) => h.id === state.selectedId)
})

const selectedEndpoint = computed(() => {
  return endpointConfigsApi.data.value.find(
    (c) => c.id === state.endpointConfigId,
  )
})

fetchInitData()

function fetchInitData() {
  historiesApi.load()
  fetchEndpointsData()
}

async function fetchEndpointsData() {
  await endpointConfigsApi.load()

  const hit = endpointConfigsApi.data.value.find(
    (c) => c.id === state.endpointConfigId,
  )
  if (!hit) {
    state.endpointConfigId = endpointConfigsApi.data.value[0]?.id
  }
}

async function selectHistory(id: number) {
  state.selectedId = id

  await historyMessageApi.load(id)
}

async function createNewChat() {
  const chatHistory = await chatHistoryTable.createOne({
    name: t('common.untitled'),
  })

  await historiesApi.load()

  await selectHistory(chatHistory.id)
}

async function updateHistoryName(name: string) {
  const selected = selectedHistory.value
  if (!selected) return

  await chatHistoryTable.updateOne({
    ...selected,
    name,
  })

  await historiesApi.load()
}

async function handleDeleteHistory(conf: IChatHistoryModel) {
  await chatHistoryTable.deleteAllById(conf.id)
  await historiesApi.load()
}
</script>

<template>
  <div class="flex h-full">
    <aside class="w-50 flex flex-col bg-light-3 overflow-auto border-(0 r solid gray-2) ">
      <div class="flex-1 overflow-auto">
        <template v-if="historiesApi.data.value.length">
          <div v-for="h in historiesApi.data.value" :key="h.id" class="p-2 cursor-pointer hover:bg-light-5"
            :class="{ 'bg-light-5': state.selectedId === h.id }" @click="selectHistory(h.id)">
            <div class="w-full flex">
              <div class="flex-1 w-0 truncate">{{ h.name || t('common.untitled') }}</div>
              <Icon class="i-carbon:trash-can cursor-pointer" @click="handleDeleteHistory(h)" />
            </div>
            <div class="text-sm text-gray-4 mt-1">
              {{ dayjs.unix(h.createdDate).format("YYYY-MM-DD HH:mm:ss") }}
            </div>
          </div>
        </template>
        <div class="size-full flex items-center justify-center">
          <Empty ></Empty>
        </div>
      </div>
      <div class="bottom-btn">
        <Button class="!rounded-0 w-full" @click="createNewChat">+ {{ t('chat.newChat') }}</Button>
      </div>
    </aside>

    <section class="flex flex-col h-full flex-1 bg-white">
      <template v-if="selectedHistory">
        <div class="flex gap-1 items-center p-2 border-(0 b solid gray-2)">
          <div class="truncate w-200px">
            <ClickToEdit :value="selectedHistory.name" @ok="updateHistoryName">
              {{ selectedHistory.name }}
              <template #edit="{ value, update }">
                <Input class="w-full" :model-value="value" @update:model-value="update" autofocus />
              </template>
            </ClickToEdit>
          </div>
          <Select class="flex-1" v-model="state.endpointConfigId" :options="endpointConfigsApi.data.value"
            :keys="{ label: 'name', value: 'id' }" :placeholder="t('chat.selectEndpoint')" />
        </div>

        <div class="flex-1 h-0">
          <ChatWithHistory :history-id="selectedHistory.id" :endpoint-config="selectedEndpoint" />
        </div>
      </template>
      <template v-else>
        <div class="flex items-center justify-center h-full text-muted">
          {{ t('chat.selectHistory') }}
        </div>
      </template>
    </section>
  </div>
</template>
