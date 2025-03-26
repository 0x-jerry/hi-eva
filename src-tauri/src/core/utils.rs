use tauri::{LogicalPosition, Manager, PhysicalPosition, WebviewWindow};

use super::VerticalMoveDir;

pub fn calc_window_position(
    win: &WebviewWindow,
    dir: Option<VerticalMoveDir>,
) -> PhysicalPosition<f64> {
    let offset_y = if dir.unwrap_or_default() == VerticalMoveDir::Up {
        -(16.0 + 36.0)
    } else {
        16.0
    };

    let app_scale_factor = win
        .app_handle()
        .primary_monitor()
        .unwrap()
        .unwrap()
        .scale_factor();

    let mouse_pos = win
        .cursor_position()
        .unwrap()
        .to_logical::<f64>(app_scale_factor);

    let offset_pos = LogicalPosition::new(-20.0, offset_y);

    let pos = LogicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);

    pos.to_physical(app_scale_factor)
}
