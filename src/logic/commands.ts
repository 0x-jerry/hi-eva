import { invoke } from '@tauri-apps/api/core'
import { snakeCase } from 'lodash-es'
import type { AppBasicConfig } from '../types'

/**
 * Opt should be an object, and should match the signatures in [commands.rs](../../src-tauri/src/commands.rs).
 */
type ICommandType = {
  getSelectedText(): Promise<string | undefined>
  applyAppearance(): Promise<void>
  hideToolbarWindow(): Promise<void>
  setChatPinned(opt: { pinned: boolean }): Promise<boolean>
  openChat(opt: { promptId: string }): Promise<void>
  openSettingFolder(): Promise<void>
  applyAutoTrigger(): Promise<void>
  applyClipboardListener(): Promise<void>
  applyGlobalShortcut(): Promise<void>
  saveConfiguration(opt: { conf: AppBasicConfig }): Promise<void>
  getConfiguration(): Promise<AppBasicConfig>
}

function buildCommands<T>(): T {
  const r = new Proxy(
    {},
    {
      get(_target, p, _receiver) {
        // biome-ignore lint/suspicious/noExplicitAny: on purpose
        return (arg: any) => invoke(snakeCase(p as string), arg)
      },
    },
  )

  return r as T
}

export const commands = buildCommands<ICommandType>()
