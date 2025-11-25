import { useLocalStorage } from '@vueuse/core'
import i18next from 'i18next'
import I18NextVue from 'i18next-vue'
import type { Plugin } from 'vue'

const I18N_NS = 'translation'

const resources = loadResources()

export const defaultLang = useLocalStorage('lang', window.navigator.language)

export const availableLangs = Object.keys(resources)

i18next.init({
  lng: defaultLang.value,
  interpolation: {
    escapeValue: false,
  },
  fallbackLng: 'en',
  resources: resources,
})

export const i18n: Plugin = (app) => {
  app.use(I18NextVue, { i18next })
}

export function applyLangUpdate() {
  i18next.changeLanguage(defaultLang.value)
}

function loadResources() {
  const langs = import.meta.glob('../**/*.json', {
    eager: true,
    import: 'default',
  })

  const langSources = Object.entries(langs).map(([key, data]) => {
    const [lang] = key.replace('.json', '').split('/').slice(2)

    return {
      lang,
      data,
    }
  })

  const resourcePairs = Object.entries(
    Object.groupBy(langSources, (item) => item.lang),
  ).map(([lang, data]) => {
    const resource = {}

    data?.forEach((item) => {
      Object.assign(resource, item.data)
    })

    return [lang, { [I18N_NS]: resource }]
  })

  const resources = Object.fromEntries(resourcePairs)

  return resources
}
