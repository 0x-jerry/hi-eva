use tauri::{Manager, WebviewWindow, WebviewWindowBuilder};

use super::MyApp;

pub trait MyAppWindowExt {
    fn get_main_window(&self) -> WebviewWindow;
    fn get_toolbar_window(&self) -> WebviewWindow;
    fn get_chat_window(&self) -> WebviewWindow;
    fn scale_factor(&self) -> f64;

    fn open_and_focus(&self, label: &str);
}

pub static MAIN_WINDOW_LABEL: &str = "main";
pub static TOOLBAR_WINDDOW_LABEL: &str = "toolbar";
pub static CHAT_WINDDOW_LABEL: &str = "chat";

impl MyAppWindowExt for MyApp {
    fn get_main_window(&self) -> WebviewWindow {
        if let Some(win) = self.get_webview_window(MAIN_WINDOW_LABEL) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            self.app(),
            MAIN_WINDOW_LABEL,
            tauri::WebviewUrl::App("#/".into()),
        )
        .inner_size(800.0, 600.0)
        .center()
        .accept_first_mouse(true)
        .visible(true);

        let win = win_builder.build().expect("Create main window failed!");

        return win;
    }

    fn get_toolbar_window(&self) -> WebviewWindow {
        if let Some(win) = self.get_webview_window(TOOLBAR_WINDDOW_LABEL) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            self.app(),
            TOOLBAR_WINDDOW_LABEL,
            tauri::WebviewUrl::App("#/toolbar".into()),
        )
        .inner_size(300.0, 60.0)
        .decorations(false)
        .resizable(false)
        .visible(false)
        .skip_taskbar(true)
        .accept_first_mouse(true)
        .focused(false)
        .always_on_top(true);

        let win = win_builder.build().expect("Create toolbar window failed!");

        return win;
    }

    fn get_chat_window(&self) -> WebviewWindow {
        if let Some(win) = self.get_webview_window(CHAT_WINDDOW_LABEL) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            self.app(),
            CHAT_WINDDOW_LABEL,
            tauri::WebviewUrl::App("#/chat".into()),
        )
        .inner_size(400.0, 600.0)
        .decorations(false)
        .resizable(false)
        .visible(false)
        .accept_first_mouse(true)
        .skip_taskbar(true)
        .always_on_top(false);

        let win = win_builder.build().expect("Create toolbar window failed!");

        return win;
    }

    fn scale_factor(&self) -> f64 {
        let win = self.get_main_window();
        let scale_factor = win.scale_factor().unwrap();
        return scale_factor;
    }

    fn open_and_focus(&self, label: &str) {
        if let Some(win) = self.get_webview_window(label) {
            win.show().unwrap();
            win.set_focus().unwrap();
        }
    }
}
