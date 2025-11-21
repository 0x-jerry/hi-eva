<script setup lang="ts">
import { useAsyncData } from '@0x-jerry/vue-kit'
import dayjs from 'dayjs'
import { computed, reactive } from 'vue'
import ChatRoom from '../../components/Chat/ChatRoom.vue'
import { chatHistoryTable } from '../../database/chatHistory'
import { chatHistoryMsgTable } from '../../database/chatHistoryMsg'

const historyApi = useAsyncData(chatHistoryTable.findAll, [])

const historyMessageApi = useAsyncData(chatHistoryMsgTable.getMsgs, [])

const state = reactive({
  selectedId: null as number | null,
})

const selectedHistory = computed(() => {
  return historyApi.data.value.find((h) => h.id === state.selectedId)
})

historyApi.load()

async function selectHistory(id: number) {
  state.selectedId = id

  await historyMessageApi.load(id)
}
</script>

<template>
  <div class="flex gap-4 h-(full) p-4">
    <!-- Left: history list -->
    <aside class="w-72 border-(1 solid gray-2) rounded p-2 bg-light-3 overflow-auto">
      <h3 class="mb-2">History</h3>
      <ul>
        <li v-for="h in historyApi.data.value" :key="h.id" class="p-2 rounded cursor-pointer hover:bg-light-5"
            :class="{ 'bg-light-5': state.selectedId === h.id }" @click="selectHistory(h.id)">
          <div class="font-medium truncate">{{ h.name || 'Untitled' }}</div>
          <div class="text-sm text-(muted)">{{ dayjs.unix(h.createdDate).format('YYYY-MM-DD HH:mm') }}</div>
        </li>
      </ul>
    </aside>

    <!-- Right: chat content -->
    <section class="flex-1 border-(1 solid gray-2) rounded p-4 bg-white">
      <template v-if="selectedHistory">
        <div class="mb-2 font-semibold">{{ selectedHistory.name }}</div>
        <ChatRoom :title="selectHistory.name" :messages="historyMessageApi.data.value" />
      </template>
      <template v-else>
        <div class="flex items-center justify-center h-full text-muted">Select a history on the left to view chat</div>
      </template>
    </section>
  </div>
</template>

