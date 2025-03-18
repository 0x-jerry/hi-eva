use tauri::{PhysicalPosition, PhysicalSize, Result, Runtime, WebviewWindow};

use super::MacWindowExt;

pub trait MyWebviewWindowExt<R: Runtime> {
    fn move_out_of_screen(&self) -> Result<()>;
    /// Check if the cursor is outside the window
    ///
    /// If the window is invisible, return false
    fn is_click_outside(&self) -> bool;

    /// Check if the cursor is in the window
    fn is_cursor_in(&self) -> bool;
}

impl<R: Runtime> MyWebviewWindowExt<R> for WebviewWindow<R> {
    fn move_out_of_screen(&self) -> Result<()> {
        if !self.is_visible()? {
            return Ok(());
        }

        let pos = PhysicalPosition::new(0.0, -9999999.0);

        self.set_position(pos)?;

        #[cfg(unix)]
        self.ns_hide()?;

        Ok(())
    }

    fn is_cursor_in(&self) -> bool {
        let cursor_pos = self.cursor_position().unwrap();

        let size: PhysicalSize<f64> = self.outer_size().unwrap().cast();
        let win_pos: PhysicalPosition<f64> = self.outer_position().unwrap().cast();

        cursor_pos.x > win_pos.x
            && cursor_pos.x < win_pos.x + size.width
            && cursor_pos.y > win_pos.y
            && cursor_pos.y < win_pos.y + size.height
    }

    fn is_click_outside(&self) -> bool {
        if !self.is_visible().unwrap() {
            return false;
        }

        return !self.is_cursor_in();
    }
}
