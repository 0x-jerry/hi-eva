use std::sync::Mutex;

#[derive(Default)]
pub struct AppStateInner {
    pub selected_text: String,
}

pub type AppState = Mutex<AppStateInner>;
