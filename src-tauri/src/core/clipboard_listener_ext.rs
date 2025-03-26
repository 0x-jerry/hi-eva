use clipboard_rs::{ClipboardWatcher, ClipboardWatcherContext};
use tauri::Manager;

use super::{AppState, MyApp};

pub trait ClipboardListenerExt {
    fn stop_clipboard_listener(&self);
    fn start_clipboard_listener(&self);
}

impl ClipboardListenerExt for MyApp {
    fn start_clipboard_listener(&self) {
        self.stop_clipboard_listener();

        let app_cloned = self.clone();

        let mut watcher = ClipboardWatcherContext::<MyApp>::new().expect("Init clipboard watcher");

        let shutdown_handler = watcher.add_handler(app_cloned).get_shutdown_channel();

        {
            let state = self.state::<AppState>();
            let mut state = state.lock().unwrap();

            state.clipboard_listener = Some(shutdown_handler);
        }

        log::info!("Start clipboard listener");

        // Watch clipboard change
        tauri::async_runtime::spawn_blocking(move || {
            watcher.start_watch();
        });
    }

    fn stop_clipboard_listener(&self) {
        log::info!("Stop clipboard listener");

        let state = self.state::<AppState>();
        let mut state = state.lock().unwrap();

        if let Some(shutdown_handler) = state.clipboard_listener.take() {
            shutdown_handler.stop();
        }
    }
}
