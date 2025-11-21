<script setup lang="ts">
import { useAsyncData } from '@0x-jerry/vue-kit'
import dayjs from 'dayjs'
import { Button, Inplace, InputText, Select } from 'primevue'
import { computed, reactive } from 'vue'
import ChatWithHistory from '../../components/ChatWithHistory.vue'
import { chatHistoryTable } from '../../database/chatHistory'
import { chatHistoryMsgTable } from '../../database/chatHistoryMsg'
import { endpointConfigTable } from '../../database/endpointConfig'

const historiesApi = useAsyncData(chatHistoryTable.findAll, [])

const historyMessageApi = useAsyncData(chatHistoryMsgTable.getMsgs, [])

const endpointConfigsApi = useAsyncData(endpointConfigTable.findAll, [])

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

historiesApi.load()
fetchEndpointsData()

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
    name: '未命名',
  })

  await historiesApi.load()

  await selectHistory(chatHistory.id)
}

async function updateHistoryName() {
  const selected = selectedHistory.value
  if (!selected) return

  await chatHistoryTable.updateOne({
    ...selected,
  })

  await historiesApi.load()
}
</script>

<template>
  <div class="flex h-full">
    <aside class="w-50 flex flex-col bg-light-3 overflow-auto">
      <div class="flex-1">
        <div v-for="h in historiesApi.data.value" :key="h.id" class="p-2 rounded cursor-pointer hover:bg-light-5"
          :class="{ 'bg-light-5': state.selectedId === h.id }" @click="selectHistory(h.id)">
          <div class="truncate">{{ h.name || "Untitled" }}</div>
          <div class="text-sm text-gray-4 mt-1">
            {{ dayjs.unix(h.createdDate).format("YYYY-MM-DD HH:mm") }}
          </div>
        </div>
      </div>
      <div class="bottom-btn">
        <Button class="!rounded-0 w-full" @click="createNewChat">+ 新建对话</Button>
      </div>
    </aside>

    <section class="flex flex-col h-full flex-1 border-(0 l solid gray-2) bg-white">
      <template v-if="selectedHistory">
        <div class="flex gap-1 items-center p-2 border-(0 b solid gray-2)">
          <div class="truncate w-200px">
            <Inplace @close="updateHistoryName">
              <template #display>
                {{ selectedHistory.name }}
              </template>
              <template #content="{ closeCallback }">
                <InputText class="w-full" v-model="selectedHistory.name" @keydown.enter.prevent.stop="closeCallback" autofocus />
              </template>
            </Inplace>
          </div>
          <Select class="flex-1" v-model="state.endpointConfigId" :options="endpointConfigsApi.data.value"
            optionLabel="name" optionValue="id" placeholder="Select a endpoint" />
        </div>

        <div class="flex-1 h-0">
          <ChatWithHistory :history-id="selectedHistory.id" :endpoint-config="selectedEndpoint" />
        </div>
      </template>
      <template v-else>
        <div class="flex items-center justify-center h-full text-muted">
          Select a history on the left to view chat
        </div>
      </template>
    </section>
  </div>
</template>
