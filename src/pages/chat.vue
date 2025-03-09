<script lang='ts' setup>
import { useUrlSearchParams } from '@vueuse/core';
import { reactive } from 'vue';
import type { ChatHistory } from '../components/Chat/types';
import ChatMessages from '../components/Chat/ChatMessages.vue';
import { nanoid } from '@0x-jerry/utils';

interface RouteParams {
  readonly type: string
}

const urlParams = useUrlSearchParams<RouteParams>('hash')

const state = reactive({
  messages: {
    id: nanoid(),
    name: 'temp',
    messages: [],
  } as ChatHistory
})
</script>

<template>
  <div class="page">
    <ChatMessages v-model="state.messages" />
    <!--  -->
  </div>
</template>

<style lang='less' scoped></style>