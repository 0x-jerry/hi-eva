use tauri::{LogicalPosition, PhysicalPosition, WebviewWindow};

pub fn calc_window_position(win: &WebviewWindow) -> PhysicalPosition<f64> {
    let offset_pos: PhysicalPosition<f64> =
        LogicalPosition::new(20.0, 16.0).to_physical(win.scale_factor().unwrap());

    let mouse_pos = win.cursor_position().unwrap();
    // todo, calc windows position
    let pos = PhysicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);
    pos
}
