use tauri::{is_dev, AppHandle};
use tauri_plugin_autostart::ManagerExt;

use crate::core::ConfigurationExt;

pub fn apply_autolaunch(app: &AppHandle) {
    let app_conf = app.get_conf();

    if is_dev() {
        return;
    }

    let autolaunch = app.autolaunch();

    if autolaunch.is_enabled().unwrap_or_default() != app_conf.autostart {
        if app_conf.autostart {
            let _ = autolaunch.enable();
        } else {
            let _ = autolaunch.disable();
        }
    }
}
