use std::str::FromStr;

use anyhow::Result;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::core::{AppState, ConfigurationExt, show_toolbar_win};

pub fn apply_global_shortcut(app: &AppHandle) -> Result<()> {
    let conf = app.get_conf();

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

            let state = app.state::<AppState>();
            let selected_text = text_selection::get_selected_text().unwrap_or_default();

            let selected_text = selected_text.trim().to_string();

            state.set_selected_text(selected_text);
            show_toolbar_win(app, None);
        })
        .map_err(|err| tauri::Error::Anyhow(err.into()))?;

    Ok(())
}
