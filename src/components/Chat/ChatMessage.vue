<script setup lang="ts">
import { Button, Tooltip } from 'tdesign-vue-next'
import { t } from '../../composables'
import type { IChatHistoryMsgItem } from '../../database/chatHistoryMsg'
import Icon from '../Icon.vue'
import Markdown from '../Markdown.vue'

export interface ChatMessageProps {
  message: IChatHistoryMsgItem
}

export interface ChatMessageEmits {
  'reset-to': [msg: IChatHistoryMsgItem]
  continue: [msg: IChatHistoryMsgItem]
  copy: [msg: IChatHistoryMsgItem]
  delete: [msg: IChatHistoryMsgItem]
}

const props = defineProps<ChatMessageProps>()
const emit = defineEmits<ChatMessageEmits>()

const roleClass = props.message.role === 'user' ? 'from-user' : 'from-assistant'
const isUser = props.message.role === 'user'

interface ToolConfig {
  event: keyof ChatMessageEmits
  icon: string
  tooltip: string
}

const tools: ToolConfig[] = [
  {
    event: 'copy',
    icon: 'i-carbon:copy',
    tooltip: t('chatMessage.copyMsg'),
  },
  {
    event: 'continue',
    icon: 'i-carbon:continue',
    tooltip: t('chatMessage.ContinueMsg'),
  },
  {
    event: 'reset-to',
    icon: 'i-carbon:restart',
    tooltip: t('chatMessage.resetMsg'),
  },
  {
    event: 'delete',
    icon: 'i-carbon:trash-can',
    tooltip: t('chatMessage.deleteMsg'),
  },
]

function handleToolEvent(tool: ToolConfig) {
  // @ts-expect-error
  emit(tool.event, props.message)
}
</script>

<template>
  <div :class="['chat-message', roleClass]">
    <div class="meta flex items-center">
      <template v-if="isUser">
        <span class="name">{{ message.role }}</span>
        <Icon class="i-carbon:user-avatar text-2xl" />
      </template>
      <template v-else>
        <Icon class="i-carbon:machine-learning-model text-2xl" />
        <span class="name">{{ message.role }}</span>
      </template>
    </div>

    <div class="content">
      <Markdown :content="message.content" />
    </div>
    <div class="content-tools">
      <template v-for="tool in tools">
        <Tooltip :content="$t(tool.tooltip)">
          <Button size="small" shape="circle" theme="default" @click="handleToolEvent(tool)">
            <Icon :class="tool.icon" />
          </Button>
        </Tooltip>
      </template>
    </div>
  </div>
</template>

<style lang="less" scoped>
.chat-message {
  max-width: 90%;
  position: relative;

  .content-tools {
    --uno: text-xs transition flex gap-2 items-center;
    margin-top: 8px;
    opacity: 0;
  }

  &:hover {
    .content-tools {
      opacity: 1;
    }
  }
}

.content {
  padding: 8px;
  border-radius: 6px;
  max-width: 100%;
  width: fit-content;
}

.from-user {
  align-self: flex-end;
  display: flex;
  flex-direction: column;
  align-items: flex-end;

  .meta {
    justify-content: end;
  }

  .content {
    background: #e6f7ff;
  }

  .content-tools {
    --uno: justify-end;
  }
}

.from-assistant {
  align-self: flex-start;

  .content {
    background: #f5f5f5;
  }
}

.meta {
  --uno: text-sm text-gray-5 mb-2;

  .name {
    &::first-letter {
      text-transform: uppercase;
    }
  }
}
</style>
