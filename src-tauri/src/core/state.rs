use std::sync::Mutex;

use clipboard_rs::WatcherShutdown;
use tauri::PhysicalSize;

#[derive(Default)]
pub struct AppStateInner {
    pub selected_text: String,
    pub chat_panel_pinned: bool,
    pub toolbar: ToolbarStateInner,
    pub clipboard_listener: Option<WatcherShutdown>,
}

#[derive(Default)]
pub struct ToolbarStateInner {
    pub size: PhysicalSize<u32>,
    pub focused: bool,
}

pub type AppState = Mutex<AppStateInner>;
