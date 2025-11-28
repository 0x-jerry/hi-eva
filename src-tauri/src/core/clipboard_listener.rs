use clipboard_rs::{Clipboard, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{AppHandle, Manager};

use crate::core::{show_toolbar_win, update_tray_menu, ConfigurationExt};

use super::AppState;

struct ClipboardApp(AppHandle);

impl ClipboardHandler for ClipboardApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            let selected_text = selected_text.trim();
            if selected_text.len() > 0 {
                {
                    let state = self.0.state::<AppState>();
                    let selected_text = selected_text.to_string();
                    state.set_selected_text(selected_text);
                }

                show_toolbar_win(&self.0, None);
            }
        }
    }
}

pub fn apply_clipboard_listener(app: &AppHandle) {
    let conf = app.get_conf();

    if conf.enable_listen_clipboard {
        start_watch_clipboard(app);
    } else {
        stop_watch_clipboard(app);
    }

    update_tray_menu(app);
}

fn start_watch_clipboard(app: &AppHandle) {
    stop_watch_clipboard(app);

    let clipboard_app = ClipboardApp(app.clone());

    let mut watcher =
        ClipboardWatcherContext::<ClipboardApp>::new().expect("Init clipboard watcher");

    let shutdown_handler = watcher.add_handler(clipboard_app).get_shutdown_channel();

    let state = app.state::<AppState>();

    state.set_clipboard_watch_handler(Some(shutdown_handler));

    tauri::async_runtime::spawn_blocking(move || {
        log::info!("Start watch clipboard");
        watcher.start_watch();
    });
}

fn stop_watch_clipboard(app: &AppHandle) {
    log::info!("Stop watch clipboard");

    let state = app.state::<AppState>();

    if let Some(shutdown_handler) = state.take_clipboard_watch_handler() {
        shutdown_handler.stop();
    }
}
