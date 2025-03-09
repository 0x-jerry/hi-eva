use clipboard_rs::{ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{
    AppHandle, Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, WebviewWindowBuilder,
};
use text_selection::{ListenResult, TextSelectionHandler};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone)]
struct MyApp {
    app: AppHandle,
}

impl ClipboardHandler for MyApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        if let Ok(result) = text_selection::get_selected_text() {
            log::info!("clipboard text is {:?}", result);

            self.on_selection_change(ListenResult {
                selected_text: result,
                mouse_position: text_selection::get_mouse_pos(),
            });
        }
    }
}

impl TextSelectionHandler for MyApp {
    fn on_selection_change(&self, result: ListenResult) {
        println!("selected: {:?}", result);

        let win = get_or_create_toolbar_window(&self.app);

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(dev)]
    dotenv::from_filename(".env.development").expect("load env failed");

    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // create toolbar window in advance.
            let _ = get_or_create_toolbar_window(&app_handle);

            let my_app = MyApp {
                app: app_handle.clone(),
            };

            let my_cloned_app = my_app.clone();
            tauri::async_runtime::spawn_blocking(move || {
                //
                let mut watcher =
                    ClipboardWatcherContext::<MyApp>::new().expect("Init clipboard watcher");
                watcher.add_handler(my_cloned_app);
                watcher.start_watch();
            });

            #[cfg(windows)]
            tauri::async_runtime::spawn_blocking(move || {
                let _ = text_selection::listen(my_app);
            });

            #[cfg(unix)]
            let _ = text_selection::listen(my_app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_or_create_toolbar_window(app: &AppHandle) -> WebviewWindow {
    let win_label = "toolbar";

    if let Some(win) = app.get_webview_window(&win_label) {
        return win;
    }

    let win_builder =
        WebviewWindowBuilder::new(app, win_label, tauri::WebviewUrl::App("#/toolbar".into()))
            .inner_size(300.0, 60.0)
            .decorations(false)
            .transparent(true)
            .resizable(false)
            .visible(false)
            .always_on_top(true);

    let win = win_builder.build().expect("Create toolbar window failed!");

    return win;
}
