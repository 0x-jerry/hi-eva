<script lang="ts">
import { marked } from 'marked'
import markedKatex from 'marked-katex-extension'

marked.use(markedKatex({ throwOnError: false }))
</script>

<script lang='ts' setup>
import { computed } from 'vue'
import 'github-markdown-css'

export interface MarkdownContentProps {
  content?: string
}

const props = defineProps<MarkdownContentProps>()

const html = computed(() => marked.parse(props.content || ''))
</script>

<template>
  <div class="markdown-body" v-html="html"></div>
</template>

<style scoped>
.markdown-body {
  --bgColor-default: transparent;
}
</style>
