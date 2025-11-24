import { Drawer } from 'tdesign-vue-next'
import { ref } from 'vue'
import { useComponentWrapper } from './useComponentWrapper'

export function useDrawer() {
  const visible = ref(false)

  const wrapperInstance = useComponentWrapper(Drawer, {
    props() {
      return {
        visible: visible.value,
        'onUpdate:visible': (val: boolean) => {
          visible.value = val
        },
      }
    },
  })

  return Object.assign(wrapperInstance, {
    open() {
      visible.value = true
    },
    close() {
      visible.value = false
    },
  })
}
