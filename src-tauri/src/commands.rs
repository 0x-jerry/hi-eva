use tauri::AppHandle;

use crate::app::MyApp;

#[tauri::command]
pub fn get_selected_text(app: AppHandle) -> Option<String> {
    let my_app = MyApp::new(app);

    my_app.get_selected_text()
}
