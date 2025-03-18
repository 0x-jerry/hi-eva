use std::{ops::Deref, sync::Mutex};

use clipboard_rs::{Clipboard, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{AppHandle, Manager};
use text_selection::SelectionRect;

use crate::{core::AppState, plugins::MyWebviewWindowExt};

use super::{
    mouse_listener, AppMessageExt, AppStateInner, AppTrayExt, MouseExtTrait, MyAppWindowExt,
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

        let app_cloned = self.clone();

        // Watch clipboard change
        tauri::async_runtime::spawn_blocking(move || {
            let mut watcher =
                ClipboardWatcherContext::<MyApp>::new().expect("Init clipboard watcher");

            watcher.add_handler(app_cloned);
            watcher.start_watch();
        });

        let app_cloned = self.clone();

        let _ = mouse_listener::listen(app_cloned);
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
                let state = self.state::<AppState>();
                let mut state = state.lock().unwrap();
                state.selected_text = selected_text.to_string();

                self.open_toolbar(None);
            }
        }
    }
}

impl MouseExtTrait for MyApp {
    fn on_selection_change(&self, result: Option<SelectionRect>) {
        if let Some(result) = result {
            log::info!("result is {:?}", result);

            let state = self.state::<AppState>();
            let mut state = state.lock().unwrap();
            state.selected_text = result.text.unwrap_or_default();

            // todo, calc window position

            self.open_toolbar(None);
        }
    }

    fn on_mouse_down(&self) {
        let win_toolbar = self.get_toolbar_window();

        if win_toolbar.is_click_outside() {
            win_toolbar.move_out_of_screen().unwrap();
        }

        if self.get_chat_window().is_click_outside() {
            self.hide_chat();
        }
    }

    fn get_cursor_position(&self) -> (f64, f64) {
        let pos = self
            .cursor_position()
            .unwrap()
            .to_logical(self.scale_factor());

        (pos.x, pos.y)
    }

    fn on_mouse_move(&self) {
        // todo
    }
}
