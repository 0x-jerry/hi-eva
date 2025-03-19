import { invoke } from '@tauri-apps/api/core'
import { snakeCase } from 'lodash-es'

/**
 * Opt should be an object, and should match the signatures in [commands.rs](../../src-tauri/src/commands.rs).
 */
type ICommandType = {
  getSelectedText(): Promise<string | undefined>
  applyAppearance(): Promise<void>
  hideToolbarWindow(): Promise<void>
  setChatPinned(opt: { pinned: boolean }): Promise<boolean>
  openChat(opt: { promptId: string }): Promise<void>
}

function buildCommands<T>(): T {
  const r = new Proxy(
    {},
    {
      get(_target, p, _receiver) {
        // biome-ignore lint/suspicious/noExplicitAny: <explanation>
        return (arg: any) => invoke(snakeCase(p as string), arg)
      },
    },
  )

  return r as T
}

export const commands = buildCommands<ICommandType>()
