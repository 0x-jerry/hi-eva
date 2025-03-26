import {
  execMigration,
  type Awaitable,
  type UpgradeConfig,
  type VersionedData,
} from '@0x-jerry/utils'
import { LazyStore } from '@tauri-apps/plugin-store'
import { tryOnUnmounted, watchPausable } from '@vueuse/core'
import { ref } from 'vue'

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
  const config = new LazyStore(fileName)

  const state = ref<T>(defaultConfig)

  const loadingState = {
    isLoading: false,
    loaded: false,
  }

  const _watcher = watchPausable(
    state,
    () => {
      console.log('changed')
      config.set(key, state.value)
    },
    {
      deep: true,
      flush: 'post',
    },
  )

  tryOnUnmounted(saveConfig)

  loadConfig()
  return state

  async function loadConfig() {
    if (loadingState.isLoading || loadingState.loaded) {
      return
    }

    loadingState.isLoading = true

    let value = await config.get(key)

    if (option?.migrations?.length) {
      value = await execMigration(value, { upgrades: option.migrations })
    }

    state.value = (value as T) ?? defaultConfig

    loadingState.isLoading = false
    loadingState.loaded = true
    option?.init?.(state.value)
  }

  async function saveConfig() {
    await config.set(key, state.value)
    await config.save()
  }
}
