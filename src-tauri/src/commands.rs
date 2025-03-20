use tauri::{Result, State, WebviewWindow};

use crate::core::{AppMessageExt, AppState, MyApp, MyAppWindowExt};

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
    #[cfg(unix)]
    {
        use crate::plugins::MacWindowExt;
        win.set_radius(10.0)?;
    }
    let _ = win;

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
pub async fn hide_toolbar_window(state: State<'_, MyApp>) -> Result<()> {
    state.hide_toolbar_win();

    Ok(())
}
