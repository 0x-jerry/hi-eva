use std::sync::Mutex;

#[derive(Default)]
pub struct AppStateInner {
    pub selected_text: String,
    pub chat_panel_pinned: bool,
    pub toolbar_panel_focused: bool,
}

pub type AppState = Mutex<AppStateInner>;
