<script setup lang="ts">
import { useEventListener } from '@vueuse/core'
import { onMounted, reactive, ref, useId } from 'vue'
import AboutSetting from '../../components/settings/AboutSetting.vue'
import BasicSetting from '../../components/settings/BasicSetting.vue'
import EndpointSetting from '../../components/settings/EndpointSetting.vue'
import PromptSetting from '../../components/settings/PromptSetting.vue'

const scrollElRef = ref<HTMLElement>()

const settings = [
  {
    label: 'Endpoint 设置',
    Component: EndpointSetting,
    id: useId(),
  },
  {
    label: 'Prompt 设置',
    Component: PromptSetting,
    id: useId(),
  },
  {
    label: '基础设置',
    Component: BasicSetting,
    id: useId(),
  },
  {
    label: '关于',
    Component: AboutSetting,
    visible: false,
    id: useId(),
  },
].filter((n) => n.visible !== false)

const state = reactive({
  activeMenu: '',
})

useEventListener(scrollElRef, 'scroll', (_evt) => {
  state.activeMenu = getActiveMenuId()
})

onMounted(() => {
  state.activeMenu = getActiveMenuId()
})

function getActiveMenuId() {
  const hit = settings.find((n) => {
    const top = document.getElementById(n.id)?.getBoundingClientRect().top ?? -1

    return top >= 0
  })

  return hit?.id || settings.at(-1)?.id || ''
}

function clickMenu(conf: { id: string }) {
  const el = document.getElementById(conf.id)
  el?.scrollIntoView({
    block: 'start',
    behavior: 'smooth',
  })
  state.activeMenu = conf.id
}
</script>
<template>
  <div class="page flex h-screen">
    <div class="sidebar pt-2">
      <div class="menu-item" v-for="conf in settings" :key="conf.id" @click="clickMenu(conf)"
        :class="{ active: state.activeMenu === conf.id }">
        {{ conf.label }}
      </div>
    </div>
    <div ref="scrollElRef" class="content flex-1 px-4 overflow-auto flex flex-col gap-4">
      <div class="section-content" v-for="conf in settings" :key="conf.id">
        <span :id="conf.id"></span>
        <component :is="conf.Component" />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sidebar {
  --uno: bg-light gap-2 border-(0 r solid light-8);

  width: 160px;
  display: flex;
  flex-direction: column;
}

.menu-item {
  --uno: px-2 text-gray-9 flex items-center py-2;
  cursor: pointer;

  text-decoration: none;

  &:hover {
    --uno: bg-light-7;
  }

  &.active {
    --uno: bg-light-8;
  }

}

.section-content {
  &:first-child {
    --uno: pt-2;
  }

  &:last-child {
    min-height: 100vh;
    --uno: pb-2;
  }
}

.icon {
  cursor: pointer;
}
</style>
