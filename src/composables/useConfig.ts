import {
  type Awaitable,
  execMigration,
  nanoid,
  type UpgradeConfig,
  type VersionedData,
} from '@0x-jerry/utils'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { LazyStore } from '@tauri-apps/plugin-store'
import { tryOnUnmounted, watchPausable } from '@vueuse/core'
import { debounce, isEqual } from 'lodash-es'
import { nextTick, ref } from 'vue'

export interface UseConfigInnerOption<T> {
  init?(data: T): Awaitable<void>
  migrations?: UpgradeConfig[]
}

export type UseConfigOption<T> = Omit<UseConfigInnerOption<T>, 'migrations'>

interface SyncEventPayload<T = unknown> {
  fileName: string
  key: string
  data: T
  id: string
}

const syncId = `${nanoid()}-${location.href}`
const SYNC_EVENT_NAME = 'store-changed'

function emitSyncEvent<T>(opt: SyncEventPayload<T>) {
  return emit(SYNC_EVENT_NAME, opt)
}

function listenStoreChange<T>(cb: (data: SyncEventPayload<T>) => void) {
  return listen(SYNC_EVENT_NAME, (evt) => {
    const payload = evt.payload as SyncEventPayload<T>

    if (payload.id === syncId) {
      return
    }

    cb(payload)
  })
}

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

  const triggerSyncEvent = debounce(() => {
    return emitSyncEvent({
      fileName,
      key,
      id: syncId,
      data: config.value,
    })
  }, 100)

  const watcher = watchPausable(
    config,
    () => {
      store.set(key, config.value)

      triggerSyncEvent()
    },
    {
      deep: true,
      initialState: 'paused',
    },
  )

  tryOnUnmounted(() => {
    state.unlistenHandle?.()
  })

  initStore()

  return config

  async function initStore() {
    await loadConfig()
    watcher.resume()

    state.unlistenHandle = await listenStoreChange(async (payload) => {
      if (payload.fileName !== fileName || payload.key !== key) {
        return
      }

      if (isEqual(payload.data, config.value)) {
        return
      }

      watcher.pause()
      config.value = payload.data
      await nextTick()
      watcher.resume()
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
    await option?.init?.(config.value)
  }
}
