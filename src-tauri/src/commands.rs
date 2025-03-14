use tauri::{Result, State, WebviewWindow};

use crate::{
    core::{AppMessageExt, AppState, MyApp},
    plugins::{MacWindowExt, MyWebviewWindowExt},
};

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
pub async fn apply_appearance(win: WebviewWindow) -> Result<()> {
    win.set_radius(10.0)?;

    Ok(())
}

#[tauri::command]
pub async fn set_chat_pinned(state: State<'_, AppState>, pinned: bool) -> Result<bool> {
    let mut state = state.lock().unwrap();

    state.chat_panel_pinned = pinned;

    log::info!("set pinned {:?}", state.chat_panel_pinned);

    Ok(state.chat_panel_pinned)
}

#[tauri::command]
pub async fn move_out_of_screen(win: WebviewWindow) -> Result<()> {
    win.move_out_of_screen()
}
