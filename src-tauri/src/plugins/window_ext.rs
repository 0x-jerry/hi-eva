use tauri::{PhysicalPosition, PhysicalSize, Runtime, WebviewWindow};

pub trait WebviewWindowExt<R: Runtime> {
    /// Check if the cursor is in the window
    fn is_cursor_in(&self) -> bool;
}

impl<R: Runtime> WebviewWindowExt<R> for WebviewWindow<R> {
    fn is_cursor_in(&self) -> bool {
        let cursor_pos = self.cursor_position().unwrap();

        let size: PhysicalSize<f64> = self.outer_size().unwrap().cast();
        let win_pos: PhysicalPosition<f64> = self.outer_position().unwrap().cast();

        cursor_pos.x < win_pos.x
            || cursor_pos.x > win_pos.x + size.width
            || cursor_pos.y < win_pos.y
            || cursor_pos.y > win_pos.y + size.height
    }
}
