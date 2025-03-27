import {
  execMigration,
  type Awaitable,
  type UpgradeConfig,
  type VersionedData,
} from '@0x-jerry/utils'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { LazyStore } from '@tauri-apps/plugin-store'
import { tryOnUnmounted, watchPausable } from '@vueuse/core'
import { isEqual } from 'lodash-es'
import { type Ref, ref } from 'vue'

export interface UseConfigInnerOption<T> {
  init?(data: T): Awaitable<void>
  migrations?: UpgradeConfig[]
}

export type UseConfigOption<T> = Omit<UseConfigInnerOption<T>, 'migrations'>

export function useConfig<T extends VersionedData>(
  fileName: string,
  key: string,
  defaultConfig: T,
  option?: UseConfigInnerOption<T>,
) {
  const store = new LazyStore(fileName)

  const config = ref<T>(defaultConfig)

  const state = {
    isLoading: false,
    loaded: false,
    unlistenHandle: null as null | UnlistenFn,
  }

  const _watcher = watchPausable(
    config,
    () => {
      store.set(key, config.value)
    },
    {
      deep: true,
      flush: 'post',
    },
  )

  tryOnUnmounted(() => {
    saveConfig()
    state.unlistenHandle?.()
  })

  initStore()

  Object.defineProperty(config, 'save', {
    get() {
      return saveConfig
    },
  })

  return config as Ref<T> & {
    save: () => Promise<void>
  }

  async function initStore() {
    await loadConfig()

    state.unlistenHandle = await store.onKeyChange(key, (newValue) => {
      if (!isEqual(newValue, config.value)) {
        config.value = newValue
      }
    })
  }

  async function loadConfig() {
    if (state.isLoading || state.loaded) {
      return
    }

    state.isLoading = true

    let value = await store.get(key)

    if (option?.migrations?.length) {
      value = await execMigration(value, { upgrades: option.migrations })
    }

    config.value = (value as T) ?? defaultConfig

    state.isLoading = false
    state.loaded = true
    option?.init?.(config.value)
  }

  async function saveConfig() {
    await store.set(key, config.value)
    await store.save()
  }
}
