use std::sync::Mutex;

#[derive(Default)]
pub struct AppStateInner {
    pub selected_text: String,
    pub chat_panel_pinned: bool,
    pub toolbar: ToolbarStateInner,
}

#[derive(Default)]
pub struct ToolbarStateInner {
    pub focused: bool,
    pub visible: bool,
}

pub type AppState = Mutex<AppStateInner>;
