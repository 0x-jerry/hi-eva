use tauri::{Manager, PhysicalSize};

use super::{AppState, MyApp, MyAppWindowExt, TOOLBAR_HIDDEN_LOWEST_Y_POS};

pub trait AppStateExt {
    fn save_toolbar_size(&self, size: PhysicalSize<u32>);
    fn get_saved_toolbar_size(&self) -> PhysicalSize<u32>;
    fn is_toolbar_visible(&self) -> bool;

    #[allow(dead_code)]
    fn set_toolbar_focused(&self, focused: bool);
    #[allow(dead_code)]
    fn is_toolbar_focused(&self) -> bool;
}

impl AppStateExt for MyApp {
    fn is_toolbar_focused(&self) -> bool {
        let state = self.state::<AppState>();
        let state = state.lock().unwrap();

        state.toolbar.focused
    }

    fn set_toolbar_focused(&self, focused: bool) {
        let state = self.state::<AppState>();
        let mut state = state.lock().unwrap();

        state.toolbar.focused = focused;
    }

    fn is_toolbar_visible(&self) -> bool {
        let win = self.get_toolbar_window();
        let y = win.outer_position().unwrap().y;

        y > TOOLBAR_HIDDEN_LOWEST_Y_POS
    }

    fn save_toolbar_size(&self, size: PhysicalSize<u32>) {
        let state = self.state::<AppState>();
        let mut state = state.lock().unwrap();
        state.toolbar.size = size;
    }

    fn get_saved_toolbar_size(&self) -> PhysicalSize<u32> {
        let state = self.state::<AppState>();
        let state = state.lock().unwrap();

        state.toolbar.size
    }
}
