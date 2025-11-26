import { isEqual } from 'lodash-es'
import { ref, toRaw } from 'vue'
import { commands } from '../logic/commands'
import { WindowEventName } from '../logic/events'
import type { AppBasicConfig } from '../types'
import { useWinEventListener } from './useWinEventListener'

export function useAppBasicConfig() {
  const conf = ref<Partial<AppBasicConfig>>({})

  useWinEventListener(WindowEventName.ConfigurationChanged, () => {
    reload()
  })

  return {
    get data() {
      return conf.value
    },
    load: reload,
    save,
  }

  async function reload() {
    const new_conf = await commands.getConfiguration()

    if (isEqual(conf.value, new_conf)) {
      return
    }

    conf.value = new_conf
  }

  async function save() {
    const new_conf = structuredClone(toRaw(conf.value)) as AppBasicConfig

    await commands.saveConfiguration({ conf: new_conf })
  }
}
