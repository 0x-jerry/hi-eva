use tauri::{AppHandle, Manager};

use crate::core::{get_toolbar_window, TOOLBAR_HIDDEN_LOWEST_Y_POS};

pub trait AppWindowExt {
    fn is_toolbar_visible(&self) -> bool;
    fn get_scale_factor(&self) -> f64;
    fn open_and_focus(&self, label: &str);
}

impl AppWindowExt for AppHandle {
    fn get_scale_factor(&self) -> f64 {
        let scale_factor = self.primary_monitor().unwrap().unwrap().scale_factor();
        return scale_factor;
    }

    fn open_and_focus(&self, label: &str) {
        if let Some(win) = self.get_webview_window(label) {
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
