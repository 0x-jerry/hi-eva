use serde::Serialize;
use tauri::{Emitter, EventTarget, Manager};

use super::{utils::calc_window_position, AppState, MyApp, MyAppWindowExt, CHAT_WINDOW_LABEL};

/// Emit messages to content view
pub trait AppMessageExt {
    fn hide_chat(&self);
    fn open_chat(&self, prompt_id: String);
}

#[derive(Debug, Serialize, Clone)]
struct OpenChatPayload {
    prompt_id: String,
    selected_text: String,
}

impl AppMessageExt for MyApp {
    fn open_chat(&self, prompt_id: String) {
        let win = self.get_chat_window();

        let state = self.state::<AppState>();
        let state = state.try_lock().unwrap();

        if !state.chat_panel_pinned {
            let pos = calc_window_position(&win);
            win.set_position(pos).unwrap();
        }

        win.emit_to(
            EventTarget::labeled(CHAT_WINDOW_LABEL),
            "show-chat",
            OpenChatPayload {
                prompt_id,
                selected_text: state.selected_text.clone(),
            },
        )
        .unwrap();

        win.show().unwrap();
        win.set_focus().unwrap();
    }

    fn hide_chat(&self) {
        let win = self.get_chat_window();

        win.emit_to(EventTarget::labeled(CHAT_WINDOW_LABEL), "hide-chat", ())
            .unwrap();
    }
}
