use std::sync::Mutex;

use clipboard_rs::WatcherShutdown;

#[derive(Default)]
pub struct AppStateInner {
    pub selected_text: String,
    pub chat_panel_pinned: bool,
    pub toolbar: ToolbarStateInner,
    pub clipboard_listener: Option<WatcherShutdown>,
}

#[derive(Default)]
pub struct ToolbarStateInner {
    pub focused: bool,
    pub visible: bool,
}

pub type AppState = Mutex<AppStateInner>;
