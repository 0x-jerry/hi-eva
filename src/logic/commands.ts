import { invoke } from '@tauri-apps/api/core'

export function getSelectedText(): Promise<string | undefined> {
	return invoke('get_selected_text')
}

export function openChatWindow(promptId: string): Promise<void> {
	return invoke('open_chat', { promptId })
}
