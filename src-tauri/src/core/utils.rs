use tauri::{AppHandle, LogicalPosition, PhysicalPosition, WebviewWindow};

use super::VerticalMoveDir;

pub fn calc_window_position(
    win: &WebviewWindow,
    dir: Option<VerticalMoveDir>,
) -> PhysicalPosition<f64> {
    let offset_x = -30.0;
    let offset_y = 16.0;
    let offset_y = if dir.unwrap_or_default() == VerticalMoveDir::Up {
        -(offset_y + 32.0)
    } else {
        offset_y
    };

    let cursor_pos = win.cursor_position().unwrap();

    let current_scale_factor = win.scale_factor().unwrap();

    #[cfg(unix)]
    let cursor_pos = {
        use tauri::Manager;
        let app_scale_factor = win
            .app_handle()
            .primary_monitor()
            .unwrap()
            .unwrap()
            .scale_factor();

        cursor_pos
            .to_logical::<f64>(app_scale_factor)
            .to_physical::<f64>(current_scale_factor)
    };

    let offset_pos =
        LogicalPosition::new(offset_x, offset_y).to_physical::<f64>(current_scale_factor);

    let pos = PhysicalPosition::new(cursor_pos.x + offset_pos.x, cursor_pos.y + offset_pos.y);

    log::info!("calc_window_position: {:?}", pos);

    pos
}

pub fn get_app_name(app: &AppHandle) -> String {
    app.config().product_name.clone().unwrap_or_default()
}

pub fn get_app_version(app: &AppHandle) -> String {
    app.config().version.clone().unwrap_or_default()
}
