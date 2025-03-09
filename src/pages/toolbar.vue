<script lang='ts' setup>
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { useTimeoutFn } from '@vueuse/core';
import { ref } from 'vue';

const win = getCurrentWindow()

const timeoutHandler = useTimeoutFn(() => {
  hideWindow()
}, 1000, {
  immediate: false
})


win.listen('show', () => {
  win.show()

  resizeWindow()
  timeoutHandler.start()
})

const el = ref<HTMLElement>()


async function resizeWindow() {
  if (!el.value) return
  const w = el.value.clientWidth;
  const h = el.value.clientHeight;

  const size = new LogicalSize(w, h)
  await win.setSize(size);
}

async function startDrag() {
  await win.startDragging()
}

async function hideWindow() {
  await win.hide()
}
</script>

<template>
  <div class="toolbar bg-white inline-flex items-center w-max" ref="el" @mouseleave="timeoutHandler.start()"
    @mouseenter="timeoutHandler.stop()">
    <span class="py-2 px-1 mr-2 cursor-move bg-gray-2" @mousedown="startDrag">
      <span class="icon i-carbon:draggable" />
    </span>
    <div>
      This is toolbar
    </div>
    <span class="cursor-pointer py-2 px-1 flex items-center">
      <span class="icon i-carbon:close" @click="hideWindow" />
    </span>
  </div>
</template> This is t

<style lang='less' scoped></style>