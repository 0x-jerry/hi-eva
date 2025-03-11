use tauri::{AppHandle, Emitter, EventTarget, LogicalPosition, PhysicalPosition};

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
        let offset_pos: PhysicalPosition<f64> =
            LogicalPosition::new(10.0, 5.0).to_physical(my_app.scale_factor());

        let mouse_pos = my_app.get_cursor_position();
        // todo, calc windows position
        let pos = PhysicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);

        win.set_position(pos).unwrap();
    }

    win.show().unwrap();
    win.set_focus().unwrap();
}
