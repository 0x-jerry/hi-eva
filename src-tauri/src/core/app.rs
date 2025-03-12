use std::{ops::Deref, sync::Mutex};

use clipboard_rs::{Clipboard, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Pixel};
use text_selection::{ListenResult, TextSelectionHandler};

use crate::plugins::WebviewWindowExt;

use super::{AppMessageExt, AppStateInner, AppTrayExt, MyAppWindowExt};

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

        let _ = self.get_toolbar_window();
        let _ = self.get_chat_window();
        let _ = self.get_main_window();

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
        let _ = text_selection::listen(app_cloned);
    }
}

impl ClipboardHandler for MyApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            if selected_text.trim().len() > 0 {
                self.on_selection_change(Some(ListenResult { selected_text }));
            }
        }
    }
}

impl TextSelectionHandler for MyApp {
    fn on_selection_change(&self, result: Option<ListenResult>) {
        if let Some(result) = result {
            let selected_text = result.selected_text.trim();
            if selected_text.len() <= 0 {
                return;
            }

            self.open_toolbar(selected_text.into());
        }
    }

    fn on_mouse_down(&self) {
        let win = self.get_toolbar_window();

        let should_hide = if !win.is_visible().unwrap() {
            true
        } else {
            win.is_cursor_in()
        };

        if should_hide {
            self.hide_toolbar();
        }
    }

    fn get_cursor_position(&self) -> (f64, f64) {
        let pos = self
            .cursor_position()
            .unwrap()
            .to_logical(self.scale_factor());

        (pos.x, pos.y)
    }
}
