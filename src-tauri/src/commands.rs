use tauri::{AppHandle, Emitter, EventTarget, PhysicalPosition};

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

    win.emit_to(EventTarget::labeled("chat"), "show-chat", prompt_id)
        .unwrap();

    {
        let mouse_pos = text_selection::get_mouse_pos();

        let offset_pos = (8.0, 4.0);

        // todo, calc windows position
        let pos = PhysicalPosition::new(mouse_pos.0 + offset_pos.0, mouse_pos.1 + offset_pos.1);

        win.set_position(pos).unwrap();
    }

    win.show().unwrap();
    win.set_focus().unwrap();
}
