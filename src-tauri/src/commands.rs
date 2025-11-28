use std::str::FromStr;

use tauri::{AppHandle, Manager, Result, State, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tauri_plugin_opener::OpenerExt;

use crate::core::{
    clipboard_listener, hide_toolbar_win, open_chat_win, show_toolbar_win, update_tray_menu,
    AppBasicConfig, AppState, ConfigurationExt,
};

#[tauri::command]
pub async fn get_selected_text(state: State<'_, AppState>) -> Result<String> {
    Ok(state.get_selected_text())
}

#[tauri::command]
pub async fn open_chat(app: AppHandle, prompt_id: String) -> Result<()> {
    let _ = open_chat_win(&app, prompt_id);

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
    state.set_chat_panel_pinned(pinned);

    Ok(pinned)
}

#[tauri::command]
pub async fn hide_toolbar_window(app: AppHandle) -> Result<()> {
    hide_toolbar_win(&app);

    Ok(())
}

#[tauri::command]
pub async fn apply_clipboard_listener(app: AppHandle) -> Result<()> {
    let _ = app.reload_conf();

    clipboard_listener::apply_clipboard_listener(&app);

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

            show_toolbar_win(app, None);
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
