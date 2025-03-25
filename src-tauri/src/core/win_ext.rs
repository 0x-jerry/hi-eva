use tauri::{Manager, PhysicalPosition, WebviewWindow, WebviewWindowBuilder};

use super::{utils::calc_window_position, AppStateExt, MyApp, VerticalMoveDir};

pub trait MyAppWindowExt {
    fn show_toolbar_win(&self, dir: Option<VerticalMoveDir>);
    fn hide_toolbar_win(&self);
    fn get_main_window(&self) -> WebviewWindow;
    fn get_toolbar_window(&self) -> WebviewWindow;
    fn get_chat_window(&self) -> WebviewWindow;
    fn scale_factor(&self) -> f64;

    fn open_and_focus(&self, label: &str);
}

pub static MAIN_WINDOW_LABEL: &str = "main";
pub static TOOLBAR_WINDOW_LABEL: &str = "toolbar";
pub static CHAT_WINDOW_LABEL: &str = "chat";

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
        .visible_on_all_workspaces(true)
        .visible(true);

        let win = win_builder.build().expect("Create main window failed!");

        let app = self.app().clone();
        win.on_window_event(move |event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let app = app.state::<MyApp>();
                log::info!("show main window");
                app.get_main_window().hide().unwrap();
            }
            _ => {}
        });

        log::info!("Create main window");

        return win;
    }

    fn get_toolbar_window(&self) -> WebviewWindow {
        if let Some(win) = self.get_webview_window(TOOLBAR_WINDOW_LABEL) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            self.app(),
            TOOLBAR_WINDOW_LABEL,
            tauri::WebviewUrl::App("#/toolbar".into()),
        )
        .inner_size(10.0, 10.0)
        .visible_on_all_workspaces(true)
        .accept_first_mouse(true)
        .decorations(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .transparent(true)
        .skip_taskbar(true)
        .focused(false)
        .always_on_top(true);

        let win = win_builder.build().expect("Create toolbar window failed!");
        log::info!("Create toolbar window");

        #[cfg(unix)]
        {
            use crate::plugins::MacWindowExt;
            let _ = win
                .to_non_active_panel()
                .expect("convert to non active panel failed");
        }

        win.show().unwrap();

        self.hide_toolbar_win();

        return win;
    }

    fn get_chat_window(&self) -> WebviewWindow {
        if let Some(win) = self.get_webview_window(CHAT_WINDOW_LABEL) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            self.app(),
            CHAT_WINDOW_LABEL,
            tauri::WebviewUrl::App("#/chat".into()),
        )
        .inner_size(400.0, 600.0)
        .focused(false)
        .decorations(false)
        .resizable(false)
        .visible(false)
        .transparent(true)
        .accept_first_mouse(true)
        .visible_on_all_workspaces(true)
        .skip_taskbar(true)
        .minimizable(false)
        .maximizable(false)
        .always_on_top(false);

        let win = win_builder.build().expect("Create toolbar window failed!");

        log::info!("Create chat window");

        return win;
    }

    fn scale_factor(&self) -> f64 {
        let scale_factor = self.primary_monitor().unwrap().unwrap().scale_factor();
        return scale_factor;
    }

    fn open_and_focus(&self, label: &str) {
        if let Some(win) = self.get_webview_window(label) {
            win.show().unwrap();
            win.set_focus().unwrap();
        }
    }

    /// Move toolbar to invisible area
    fn hide_toolbar_win(&self) {
        let pos = PhysicalPosition::new(0.0, -9999999.0);

        let win = self.get_toolbar_window();
        win.set_position(pos).unwrap();

        #[cfg(unix)]
        {
            use crate::plugins::MacWindowExt;
            win.ns_resign_focus().unwrap();
        }

        self.set_toolbar_focused(false);
        self.set_toolbar_visible(false);
    }

    fn show_toolbar_win(&self, dir: Option<VerticalMoveDir>) {
        let win = self.get_toolbar_window();

        let pos = calc_window_position(&win, dir);

        win.set_position(pos).unwrap();

        self.set_toolbar_visible(true);
    }
}
