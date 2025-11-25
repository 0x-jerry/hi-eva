import enConfig from 'tdesign-vue-next/es/locale/en_US'
import zhConfig from 'tdesign-vue-next/es/locale/zh_CN'
import { computed } from 'vue'
import { defaultLang } from './i18n'

const configMap: Record<string, Object> = {
  zh: zhConfig,
  en: enConfig,
}

export const tDesignConfig = computed(() => {
  return configMap[defaultLang.value] || {}
})
