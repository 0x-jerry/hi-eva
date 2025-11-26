/** biome-ignore-all lint/suspicious/noConfusingVoidType: types only */
import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event'

export interface IShowChatPayload {
  prompt_id: string
  selected_text: string
}

export enum WindowEventName {
  ChatShow = 'chat.show',
  ChatHide = 'chat.hide',
  ToolbarShow = 'toolbar:show',
  ConfigurationChanged = 'configuration:changed',
}

export interface TauriWindowEventMap {
  [WindowEventName.ChatShow]: IShowChatPayload
  [WindowEventName.ChatHide]: void
  [WindowEventName.ToolbarShow]: void
  [WindowEventName.ConfigurationChanged]: void
}

declare module '@tauri-apps/api/window' {
  interface Window {
    listen<Event extends keyof TauriWindowEventMap>(
      event: `${Event}`,
      handler: EventCallback<TauriWindowEventMap[Event]>,
    ): Promise<UnlistenFn>
  }
}
