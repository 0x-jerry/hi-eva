use tauri::{LogicalPosition, Manager, PhysicalPosition, WebviewWindow};

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

    let current_monitor_scale_factor = win
        .app_handle()
        .monitor_from_point(cursor_pos.x, cursor_pos.y)
        .unwrap()
        .unwrap_or(win.app_handle().primary_monitor().unwrap().unwrap())
        .scale_factor();

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
            .to_physical::<f64>(current_monitor_scale_factor)
    };

    let offset_pos =
        LogicalPosition::new(offset_x, offset_y).to_physical::<f64>(current_monitor_scale_factor);

    let pos = PhysicalPosition::new(cursor_pos.x + offset_pos.x, cursor_pos.y + offset_pos.y);

    log::info!("calc_window_position: {:?}", pos);

    pos
}
