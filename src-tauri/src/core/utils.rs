use tauri::{LogicalPosition, PhysicalPosition, WebviewWindow};

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

    let offset_pos: PhysicalPosition<f64> =
        LogicalPosition::new(-20.0, offset_y).to_physical(win.scale_factor().unwrap());

    let mouse_pos = win.cursor_position().unwrap();
    // todo, calc windows position
    let pos = PhysicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);
    pos
}
