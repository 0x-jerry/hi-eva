<script lang='ts' setup>
import { marked } from 'marked'
import { computed } from 'vue'
import 'github-markdown-css'

export interface MarkdownContentProps {
  content?: string
}

const props = defineProps<MarkdownContentProps>()

const html = computed(() => marked(props.content || ''))
</script>

<template>
  <div class="markdown-body" v-html="html"></div>
</template>

<style scoped>
.markdown-body {
  --bgColor-default: transparent;
}
</style>
