use tauri::{AppHandle, Emitter, EventTarget, Manager};

use crate::core::{TOOLBAR_HIDDEN_LOWEST_Y_POS, constant::event_name, get_toolbar_window};

pub trait AppWindowExt {
    fn is_toolbar_visible(&self) -> bool;
    fn open_and_focus(&self, label: &str);
}

impl AppWindowExt for AppHandle {
    fn open_and_focus(&self, label: &str) {
        if let Some(win) = self.get_webview_window(label) {
            win.emit_to(
                EventTarget::labeled(win.label()),
                event_name::WINDOW_SHOW,
                (),
            )

            .unwrap();
            win.show().unwrap();
            win.set_focus().unwrap();
        }
    }

    fn is_toolbar_visible(&self) -> bool {
        let win = get_toolbar_window(self);
        let y = win.outer_position().unwrap().y;

        y > TOOLBAR_HIDDEN_LOWEST_Y_POS
    }
}
