use std::str::FromStr;

use tauri::{AppHandle, Manager, Result, State, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_opener::OpenerExt;

use crate::core::{
    AppBasicConfig, AppMessageExt, AppState, ConfigurationExt, MyApp, MyAppWindowExt, update_tray_menu
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
pub async fn hide_toolbar_window(app: State<'_, MyApp>) -> Result<()> {
    app.hide_toolbar_win();

    Ok(())
}

#[tauri::command]
pub async fn apply_clipboard_listener(app: State<'_, MyApp>) -> Result<()> {
    let _ = app.reload_conf();

    app.apply_clipboard_listener();

    Ok(())
}

#[tauri::command]
pub async fn apply_auto_trigger(app: AppHandle) -> Result<()> {
    let _ = app.reload_conf();

    update_tray_menu(&app);

    Ok(())
}

#[tauri::command]
pub async fn open_setting_folder(app: AppHandle) -> Result<()> {
    let conf_dir = app.path().app_config_dir()?;

    log::info!("conf dir: {:?}", conf_dir);
    app.opener()
        .open_path(conf_dir.to_str().unwrap(), None::<&str>)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn apply_global_shortcut(app: AppHandle) -> Result<()> {
    let conf = app.reload_conf();

    app.global_shortcut()
        .unregister_all()
        .map_err(|err| tauri::Error::Anyhow(err.into()))?;

    if !conf.enable_global_shortcut {
        return Ok(());
    }

    if conf.global_shortcut.trim().is_empty() {
        return Ok(());
    }

    let shortcut = Shortcut::from_str(&conf.global_shortcut.trim())
        //
        .map_err(|err| tauri::Error::Anyhow(err.into()))?;

    app.global_shortcut()
        .on_shortcut(shortcut, |app, _shortcut, _evt| {
            log::info!("shortcut triggered!");

            app.state::<MyApp>().show_toolbar_win(None);
        })
        .map_err(|err| tauri::Error::Anyhow(err.into()))?;

    Ok(())
}


#[tauri::command]
pub async fn get_configuration(app: AppHandle) -> Result<AppBasicConfig> {
    let conf = app.get_conf();

    Ok(conf)
}

#[tauri::command]
pub async fn save_configuration(app: AppHandle, conf: AppBasicConfig) -> Result<()> {
    app.save_conf(&conf);

    Ok(())
}
