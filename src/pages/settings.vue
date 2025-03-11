<script setup lang="ts">
import { useScroll } from '@vueuse/core'
import { computed, ref, useId } from 'vue'
import AISetting from '../components/settings/AISetting.vue'
import AboutSetting from '../components/settings/AboutSetting.vue'
import CommonSetting from '../components/settings/CommonSetting.vue'
import PromptSetting from '../components/settings/PromptSetting.vue'

const scrollElRef = ref<HTMLElement>()

const settings = [
  {
    label: '通用设置',
    Component: CommonSetting,
    visible: false,
    id: useId(),
  },
  {
    label: 'AI 设置',
    Component: AISetting,
    id: useId(),
  },
  {
    label: 'Prompt 设置',
    Component: PromptSetting,
    id: useId(),
  },
  {
    label: '关于',
    Component: AboutSetting,
    visible: false,
    id: useId(),
  },
].filter((n) => n.visible !== false)

const activeMenu = computed(() => {
  const top = scroll.y.value

  const hit = settings.find((n) => {
    const el = document.getElementById(n.id)
    if (!el) {
      return false
    }

    if (top > el.clientTop) {
      return true
    }
  })

  return hit?.id || settings.at(-1)?.id
})

const scroll = useScroll(scrollElRef)

function clickMenu(conf: { id: string }) {
  const el = document.getElementById(conf.id)
  el?.scrollIntoView({
    behavior: 'smooth',
    block: 'start',
  })
}
</script>
<template>
  <div class="page flex h-screen">
    <div class="sidebar">
      <div class="menu-item mb-4 justify-center font-mono bg-light-8 h-8">
        <span>Hi Eva</span>
      </div>
      <div class="menu-item" v-for="conf in settings" :id="conf.id" @click="clickMenu(conf)"
        :class="{ active: activeMenu === conf.id }">
        {{ conf.label }}
      </div>
    </div>
    <div ref="scrollElRef" class="content flex-1 px-4 overflow-auto flex flex-col gap-4">
      <div class="section-content" v-for="conf in settings">
        <span :id="conf.id"></span>
        <component :is="conf.Component" />
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
.sidebar {
  --uno: bg-light gap-2 border-(0 r solid light-8);

  width: 180px;
  display: flex;
  flex-direction: column;
}

.menu-item {
  --uno: px-2 text-gray-9 flex items-center;
  cursor: pointer;

  text-decoration: none;
}

.section-content {
  &:first-child {
    @apply pt-2;
  }

  &:last-child {
    min-height: 100vh;
    @apply pb-2;
  }
}

.icon {
  cursor: pointer;
}
</style>
