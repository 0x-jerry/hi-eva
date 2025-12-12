<script lang="ts">
import { marked } from 'marked'
import markedKatex from 'marked-katex-extension'
import 'katex/dist/katex.min.css'

marked.use(markedKatex({ throwOnError: false }))
</script>

<script lang='ts' setup>
import remend from 'remend'
import { computed } from 'vue'
import 'github-markdown-css'

export interface MarkdownContentProps {
  content?: string
}

const props = defineProps<MarkdownContentProps>()

const html = computed(() => {
  const content = remend(props.content?.trim() || '')
  return marked.parse(content)
})
</script>

<template>
  <div class="markdown-body" v-html="html"></div>
</template>

<style lang="less" scoped>
.markdown-body {
  --bgColor-default: transparent;
}
</style>
