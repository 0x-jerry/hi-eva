<script setup lang="ts">
import { useEventListener } from '@vueuse/core'
import { onMounted, reactive, ref, useId } from 'vue'
import BasicSetting from '../../components/settings/BasicSetting.vue'
import PromptSetting from '../../components/settings/PromptSetting.vue'
import ProviderSetting from '../../components/settings/ProviderSetting.vue'
import { t } from '../../composables'

const scrollElRef = ref<HTMLElement>()

const settings = [
  {
    label: t('main.basicSettings'),
    Component: BasicSetting,
    id: useId(),
  },
  {
    label: t('main.providerSettings'),
    Component: ProviderSetting,
    id: useId(),
  },
  {
    label: t('main.promptSettings'),
    Component: PromptSetting,
    id: useId(),
  },
]

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
    <div class="sidebar">
      <div class="menu-item" v-for="conf in settings" :key="conf.id" @click="clickMenu(conf)"
        :class="{ active: state.activeMenu === conf.id }">
        {{ $t(conf.label) }}
      </div>
    </div>
    <div ref="scrollElRef" class="content bg-white flex-1 p-4 pt-2 overflow-auto flex flex-col gap-4">
      <div class="section-content" v-for="conf in settings" :key="conf.id">
        <span :id="conf.id"></span>
        <component :is="conf.Component" />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.sidebar {
  --uno: bg-light-3 border-(0 r solid light-8) w-50;

  display: flex;
  flex-direction: column;
}

.menu-item {
  --uno: p-2 text-gray-9 flex items-center py-2;
  cursor: pointer;

  text-decoration: none;

  &:hover {
    --uno: bg-light-5;
  }

  &.active {
    --uno: bg-light-5;
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
