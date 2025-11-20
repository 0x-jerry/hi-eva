<script setup lang="ts">
import dayjs from 'dayjs'
import { computed, onMounted, reactive, ref } from 'vue'
import ChatMessages from '../../components/Chat/ChatMessages.vue'
import type { ChatMessageWithDb } from '../../components/Chat/types'
import type { IChatHistoryModel } from '../../database/chatHistory'
import { chatHistoryTable } from '../../database/chatHistory'

// --- Repository (data access) - single responsibility
async function fetchHistories(page = 0, size = 100) {
  return await chatHistoryTable.page({ current: page, size })
}

async function fetchHistoryById(id: number) {
  return await chatHistoryTable.getAllById(id)
}

// --- Presentation state
const histories = ref<IChatHistoryModel[]>([])
const selectedId = ref<number | null>(null)
const selectedHistory = ref<IChatHistoryModel | null>(null)

// Chat model expected by `ChatMessages`
const chatModel = reactive<{ id: string; name: string; messages: ChatMessageWithDb[] }>({ id: '', name: '', messages: [] })

const promptId = computed(() => (selectedHistory.value ? String(selectedHistory.value.promptConfigId) : ''))

function normalizeRole(r: string): ChatMessageWithDb['role'] {
  if (r === 'user' || r === 'assistant' || r === 'system') return r
  return 'user'
}

function mapDbToChatModel(h: IChatHistoryModel) {
  const messages: ChatMessageWithDb[] = (h.messages || []).map((m) => {
    const role = normalizeRole(String(m.role))
    if (role === 'assistant') {
      return { role: 'assistant', content: m.content, __dbId: m.id }
    }
    if (role === 'system') {
      return { role: 'system', content: m.content, __dbId: m.id }
    }

    return { role: 'user', content: m.content, __dbId: m.id }
  })

  return {
    id: String(h.id),
    name: h.title || 'Untitled',
    messages,
  }
}

async function loadHistories() {
  const res = await fetchHistories()
  histories.value = res || []
}

async function selectHistory(id: number) {
  selectedId.value = id
  const h = await fetchHistoryById(id)
  selectedHistory.value = h || null

  if (h) {
    const mapped = mapDbToChatModel(h)
    chatModel.id = mapped.id
    chatModel.name = mapped.name
    chatModel.messages = mapped.messages
  } else {
    chatModel.id = ''
    chatModel.name = ''
    chatModel.messages = []
  }
}

onMounted(() => {
  loadHistories()
})
</script>

<template>
  <div class="flex gap-4 h-(full) p-4">
    <!-- Left: history list -->
    <aside class="w-72 border-(1 solid gray-2) rounded p-2 bg-light-3 overflow-auto">
      <h3 class="mb-2">History</h3>
      <ul>
        <li v-for="h in histories" :key="h.id" class="p-2 rounded cursor-pointer hover:bg-light-5"
            :class="{ 'bg-light-5': selectedId === h.id }" @click="selectHistory(h.id)">
          <div class="font-medium truncate">{{ h.title || 'Untitled' }}</div>
          <div class="text-sm text-(muted)">{{ dayjs.unix(h.createdDate).format('YYYY-MM-DD HH:mm') }}</div>
        </li>
      </ul>
    </aside>

    <!-- Right: chat content -->
    <section class="flex-1 border-(1 solid gray-2) rounded p-4 bg-white">
      <template v-if="selectedHistory">
        <div class="mb-2 font-semibold">{{ selectedHistory.title }}</div>
        <ChatMessages v-model="chatModel" :promptId="promptId" />
      </template>
      <template v-else>
        <div class="flex items-center justify-center h-full text-muted">Select a history on the left to view chat</div>
      </template>
    </section>
  </div>
</template>

<!-- styles intentionally omitted; page relies on existing utility classes -->
