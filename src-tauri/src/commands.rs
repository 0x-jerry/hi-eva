use tauri::{Result, State, WebviewWindow};
use tauri_nspanel::{
    cocoa::{
        appkit::{NSView, NSWindow},
        base::id,
    },
    objc::{msg_send, sel, sel_impl},
};

use crate::core::{AppMessageExt, AppState, MyApp};

#[tauri::command]
pub async fn get_selected_text(state: State<'_, AppState>) -> Result<String> {
    let state = state.lock().unwrap();

    Ok(state.selected_text.clone())
}

#[tauri::command]
pub async fn open_chat(app: State<'_, MyApp>, prompt_id: String) -> Result<()> {
    let _ = app.open_chat(prompt_id);

    Ok(())
}

#[tauri::command]
pub async fn apply_appearance(win: WebviewWindow) {
    #[cfg(unix)]
    {
        let win: id = win.ns_window().unwrap() as _;

        unsafe {
            let view: id = win.contentView();

            view.wantsLayer();

            let layer: id = view.layer();

            let _: () = msg_send![layer, setCornerRadius: 10.0];
        }
    }
}
