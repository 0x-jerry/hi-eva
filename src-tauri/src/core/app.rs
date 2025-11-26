use std::{ops::Deref, sync::Mutex};

use clipboard_rs::{Clipboard, ClipboardHandler};
use tauri::{AppHandle, Manager};

use crate::{
    core::{update_tray_menu, AppState, ConfigurationExt},
    plugins::MyWebviewWindowExt,
};

use super::{
    mouse_listener, AppMessageExt, AppStateExt, AppStateInner, AppTrayExt, ClipboardListenerExt,
    MouseExtTrait, MyAppWindowExt, SelectionResult, MAIN_WINDOW_LABEL,
};

#[derive(Clone)]
pub struct MyApp(AppHandle);

impl Deref for MyApp {
    type Target = AppHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MyApp {
    pub fn new(app: AppHandle) -> MyApp {
        MyApp(app)
    }

    pub fn app(&self) -> &AppHandle {
        &self.0
    }

    pub fn init(&self) {
        self.manage(Mutex::new(AppStateInner::default()));

        let _ = self.get_main_window();
        let _ = self.get_toolbar_window();
        let _ = self.get_chat_window();

        let _ = self.create_tray();

        self.apply_clipboard_listener();

        let app_cloned = self.clone();

        self.open_and_focus(MAIN_WINDOW_LABEL);

        let _ = mouse_listener::listen(app_cloned);

        let _ = text_selection::init();
    }

    pub fn apply_clipboard_listener(&self) {
        let conf = self.get_conf();

        if conf.enable_listen_clipboard {
            self.start_clipboard_listener();
        } else {
            self.stop_clipboard_listener();
        }

        update_tray_menu(self);
    }
}

impl ClipboardHandler for MyApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            let selected_text = selected_text.trim();
            if selected_text.len() > 0 {
                {
                    let state = self.state::<AppState>();
                    let mut state = state.lock().unwrap();
                    state.selected_text = selected_text.to_string();
                }

                self.show_toolbar_win(None);
            }
        }
    }
}

impl MouseExtTrait for MyApp {
    fn on_selection_change(&self, result: Option<SelectionResult>) {
        if !self.get_conf().enable_auto_trigger {
            return;
        }

        if let Some(result) = result {
            log::info!("result is {:?}", result);

            if result.rect.text.clone().is_some_and(|x| x.len() > 0) {
                {
                    let state = self.state::<AppState>();
                    let mut state = state.lock().unwrap();
                    state.selected_text = result.rect.text.unwrap_or_default();
                }

                self.show_toolbar_win(Some(result.mouse_move_dir));
            }
        }
    }

    fn get_cursor_position(&self) -> (f64, f64) {
        let pos = self
            .cursor_position()
            .unwrap()
            .to_logical(self.scale_factor());

        (pos.x, pos.y)
    }

    fn on_mouse_down(&self) {
        let win_toolbar = self.get_toolbar_window();

        if self.is_toolbar_visible() && !win_toolbar.is_cursor_in() {
            self.hide_toolbar_win();
        }

        if self.get_chat_window().is_click_outside() {
            self.hide_chat();
        }
    }

    fn on_mouse_move(&self) {
        #[cfg(unix)]
        {
            use crate::plugins::MacWindowExt;

            if !self.is_toolbar_visible() {
                return;
            }

            let win_toolbar = self.get_toolbar_window();

            if win_toolbar.is_cursor_in() {
                win_toolbar.ns_focus().unwrap();
            } else {
                win_toolbar.ns_resign_focus().unwrap();
            }
        }
    }
}
