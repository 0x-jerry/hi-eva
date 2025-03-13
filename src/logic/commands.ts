import { invoke } from '@tauri-apps/api/core'
import { snakeCase } from 'lodash-es'

interface ICommandType {
  getSelectedText(): Promise<string | undefined>
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
