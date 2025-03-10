use tauri::AppHandle;

use crate::app::MyApp;

#[tauri::command]
pub fn get_selected_text(app: AppHandle) -> Option<String> {
    let my_app = MyApp::new(app);

    my_app.get_selected_text()
}

#[tauri::command]
pub fn open_chat(app: AppHandle, prompt_id: String) {
    let my_app = MyApp::new(app);

    let win = my_app.get_or_create_chat_window(prompt_id);

    win.show().unwrap();
}
