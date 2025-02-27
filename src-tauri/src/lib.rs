use tauri::{
    AppHandle, Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, WebviewWindowBuilder,
};
use text_selection::ListenResult;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // create toolbar window in advance.
            let _ = get_or_create_toolbar_window(&app_handle);

            let text_selection_callback = move |result: ListenResult| {
                println!("selected: {:?}", result);

                let win = get_or_create_toolbar_window(&app_handle);

                let offset_pos = (8.0, 4.0);

                // todo, calc windows position
                let pos = LogicalPosition::new(
                    result.mouse_position.0 + offset_pos.0,
                    result.mouse_position.1 + offset_pos.1,
                );

                win.set_position(pos).unwrap();

                app_handle
                    .emit_to(EventTarget::labeled("toolbar"), "show", ())
                    .expect("Notify toolbar window");
            };

            #[cfg(windows)]
            tauri::async_runtime::spawn_blocking(move || {
                let _ = text_selection::listen(text_selection_callback);
            });

            #[cfg(unix)]
            let _ = text_selection::listen(text_selection_callback);

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
