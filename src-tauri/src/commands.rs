use tauri::{AppHandle, Emitter, EventTarget};

use crate::app::MyApp;

#[tauri::command]
pub async fn get_selected_text(app: AppHandle) -> Option<String> {
    let my_app = MyApp::new(app);

    my_app.get_selected_text()
}

#[tauri::command]
pub async fn open_chat(app: AppHandle, prompt_id: String) {
    let my_app = MyApp::new(app);

    let win = my_app.get_or_create_chat_window();

    win.emit_to(EventTarget::labeled("chat"), "prompt-id-changed", prompt_id)
        .unwrap();

    win.set_always_on_top(true).unwrap();
    win.show().unwrap();
    win.set_focus().unwrap();

    win.set_always_on_top(false).unwrap();
}
