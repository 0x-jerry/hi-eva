use std::str::FromStr;

use anyhow::Result;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::core::{show_toolbar_win, ConfigurationExt};

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

            show_toolbar_win(app, None);
        })
        .map_err(|err| tauri::Error::Anyhow(err.into()))?;

    Ok(())
}
