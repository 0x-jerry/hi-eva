import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event'

export interface IShowChatPayload {
  prompt_id: string
  selected_text: string
}

export enum WindowEventName {
  ChatShow = 'chat:show',
  ChatHide = 'chat:hide',
  ToolbarShow = 'toolbar:show',
}

declare module '@tauri-apps/api/window' {
  interface Window {
    listen(
      event: `${WindowEventName.ChatShow}`,
      handler: EventCallback<IShowChatPayload>,
    ): Promise<UnlistenFn>

    listen(
      event: `${WindowEventName.ChatHide}`,
      handler: EventCallback<void>,
    ): Promise<UnlistenFn>

    listen(
      event: `${WindowEventName.ToolbarShow}`,
      handler: EventCallback<void>,
    ): Promise<UnlistenFn>
  }
}
