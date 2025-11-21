import type { EventCallback, UnlistenFn } from '@tauri-apps/api/event'

export interface IShowChatPayload {
  prompt_id: string
  selected_text: string
}

export enum WindowEventName {
  ShowChat = 'show-chat',
  HideChat = 'hide-chat',
}

declare module '@tauri-apps/api/window' {
  interface Window {
    listen(
      event: `${WindowEventName.ShowChat}`,
      handler: EventCallback<IShowChatPayload>,
    ): Promise<UnlistenFn>

    listen(
      event: `${WindowEventName.HideChat}`,
      handler: EventCallback<void>,
    ): Promise<UnlistenFn>
  }
}
