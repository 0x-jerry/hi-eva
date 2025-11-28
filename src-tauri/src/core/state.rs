use std::{ops::Deref, sync::Mutex};

use clipboard_rs::WatcherShutdown;
use tauri::PhysicalSize;

#[derive(Default)]
pub struct AppStateInner {
    selected_text: String,
    chat_panel_pinned: bool,
    toolbar: ToolbarStateInner,
    clipboard_watcher_handler: Option<WatcherShutdown>,
}

#[derive(Default)]
pub struct ToolbarStateInner {
    #[allow(dead_code)]
    pub size: PhysicalSize<u32>,
    pub focused: bool,
}

#[derive(Default)]
pub struct AppState(Mutex<AppStateInner>);

impl Deref for AppState {
    type Target = Mutex<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppState {
    pub fn is_toolbar_focused(&self) -> bool {
        let state = self.lock().unwrap();

        state.toolbar.focused
    }

    pub fn set_toolbar_focused(&self, focused: bool) {
        let mut state = self.lock().unwrap();

        state.toolbar.focused = focused;
    }

    pub fn set_clipboard_watch_handler(&self, handler: Option<WatcherShutdown>) {
        let mut state = self.lock().unwrap();
        state.clipboard_watcher_handler = handler;
    }

    pub fn take_clipboard_watch_handler(&self) -> Option<WatcherShutdown> {
        let mut state = self.lock().unwrap();
        state.clipboard_watcher_handler.take()
    }

    pub fn get_selected_text(&self) -> String {
        let state = self.lock().unwrap();
        state.selected_text.clone()
    }

    pub fn set_selected_text(&self, new_text: String) {
        let mut state = self.lock().unwrap();
        state.selected_text = new_text
    }

    pub fn is_chat_panel_pinned(&self) -> bool {
        let state = self.lock().unwrap();
        state.chat_panel_pinned
    }

    pub fn set_chat_panel_pinned(&self, chat_panel_pinned: bool) {
        let mut state = self.lock().unwrap();
        state.chat_panel_pinned = chat_panel_pinned;
    }
}
