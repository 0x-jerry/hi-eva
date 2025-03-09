use clipboard_rs::{Clipboard, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{
    AppHandle, Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, WebviewWindowBuilder,
};
use text_selection::{ListenResult, TextSelectionHandler};

#[derive(Clone)]
pub struct MyApp {
    app: AppHandle,
}

impl MyApp {
    pub fn new(app: AppHandle) -> MyApp {
        MyApp { app }
    }

    pub fn init(&self) {
        let main_win = self.create_main_window();

        let _ = self.get_or_create_toolbar_window();

        let app_cloned = self.clone();
        tauri::async_runtime::spawn_blocking(move || {
            //
            let mut watcher =
                ClipboardWatcherContext::<MyApp>::new().expect("Init clipboard watcher");
            watcher.add_handler(app_cloned);
            watcher.start_watch();
        });

        let app_cloned = self.clone();
        #[cfg(windows)]
        tauri::async_runtime::spawn_blocking(move || {
            let _ = text_selection::listen(app_cloned);
        });

        #[cfg(unix)]
        let _ = text_selection::listen(app_cloned);
    }

    fn create_main_window(&self) -> WebviewWindow {
        let win_label = "main";

        if let Some(win) = self.app.get_webview_window(&win_label) {
            return win;
        }

        let win_builder =
            WebviewWindowBuilder::new(&self.app, win_label, tauri::WebviewUrl::App("#/".into()))
                .inner_size(800.0, 600.0)
                .center()
                .visible(true);

        let win = win_builder.build().expect("Create main window failed!");

        return win;
    }

    fn get_or_create_toolbar_window(&self) -> WebviewWindow {
        let win_label = "toolbar";

        if let Some(win) = self.app.get_webview_window(&win_label) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            &self.app,
            win_label,
            tauri::WebviewUrl::App("#/toolbar".into()),
        )
        .inner_size(300.0, 60.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .visible(false)
        .always_on_top(true);

        let win = win_builder.build().expect("Create toolbar window failed!");

        return win;
    }
}

impl ClipboardHandler for MyApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            if selected_text.trim().len() > 0 {
                self.on_selection_change(ListenResult {
                    selected_text,
                    mouse_position: text_selection::get_mouse_pos(),
                });
            }
        }
    }
}

impl TextSelectionHandler for MyApp {
    fn on_selection_change(&self, result: ListenResult) {
        println!("selected: {:?}", result);

        let win = self.get_or_create_toolbar_window();

        let offset_pos = (8.0, 4.0);

        // todo, calc windows position
        let pos = LogicalPosition::new(
            result.mouse_position.0 + offset_pos.0,
            result.mouse_position.1 + offset_pos.1,
        );

        win.set_position(pos).unwrap();

        self.app
            .emit_to(EventTarget::labeled("toolbar"), "show", ())
            .expect("Notify toolbar window");
    }
}
