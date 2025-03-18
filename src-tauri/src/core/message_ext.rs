use serde::Serialize;
use tauri::{Emitter, EventTarget, LogicalPosition, Manager, PhysicalPosition, WebviewWindow};

use super::{AppState, MyApp, MyAppWindowExt, CHAT_WINDOW_LABEL};

pub trait AppMessageExt {
    fn open_toolbar(&self, position: Option<PhysicalPosition<f64>>);
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
        let state = state.lock().unwrap();

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

    fn open_toolbar(&self, position: Option<PhysicalPosition<f64>>) {
        let win = self.get_toolbar_window();

        let pos = position.unwrap_or(calc_window_position(&win));

        win.set_position(pos).unwrap();
        win.set_always_on_top(true).unwrap();
    }
}

fn calc_window_position(win: &WebviewWindow) -> PhysicalPosition<f64> {
    let offset_pos: PhysicalPosition<f64> =
        LogicalPosition::new(20.0, 16.0).to_physical(win.scale_factor().unwrap());

    let mouse_pos = win.cursor_position().unwrap();
    // todo, calc windows position
    let pos = PhysicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);
    pos
}
