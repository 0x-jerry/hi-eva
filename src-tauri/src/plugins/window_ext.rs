use tauri::{Runtime, WebviewWindow};

pub trait MyWebviewWindowExt<R: Runtime> {
    /// Check if the cursor is outside the window
    ///
    /// If the window is invisible, return false
    fn is_click_outside(&self) -> bool;

    /// Check if the cursor is in the window
    fn is_cursor_in(&self) -> bool;
}

impl<R: Runtime> MyWebviewWindowExt<R> for WebviewWindow<R> {
    fn is_cursor_in(&self) -> bool {
        let app_scale_factor = self
        .app_handle()
        .primary_monitor()
        .unwrap()
        .unwrap()
        .scale_factor();

        let win_scale_factor = self.scale_factor().unwrap();

        let cursor_pos = self
            .cursor_position()
            .unwrap()
            .to_logical::<f64>(app_scale_factor)
            .to_physical::<f64>(win_scale_factor);

        let size = self.outer_size().unwrap().cast::<f64>();

        let win_pos = self.outer_position().unwrap().cast::<f64>();

        let is_in = cursor_pos.x > win_pos.x
            && cursor_pos.x < win_pos.x + size.width
            && cursor_pos.y > win_pos.y
            && cursor_pos.y < win_pos.y + size.height;

        // log::info!("is_cursor_in: {:?}", is_in);

        is_in
    }

    fn is_click_outside(&self) -> bool {
        if !self.is_visible().unwrap() {
            return false;
        }

        return !self.is_cursor_in();
    }
}
