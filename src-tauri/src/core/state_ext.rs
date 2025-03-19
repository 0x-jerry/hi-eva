use tauri::Manager;

use super::{AppState, MyApp};

pub trait AppStateExt {
    fn set_toolbar_visible(&self, visible: bool);
    fn is_toolbar_visible(&self) -> bool;

    #[allow(dead_code)]
    fn set_toolbar_focused(&self, focused: bool);
    #[allow(dead_code)]
    fn is_toolbar_focused(&self) -> bool;
}

impl AppStateExt for MyApp {
    fn is_toolbar_focused(&self) -> bool {
        let state = self.state::<AppState>();
        let state = state.try_lock().unwrap();

        state.toolbar.focused
    }

    fn set_toolbar_focused(&self, focused: bool) {
        let state = self.state::<AppState>();
        let mut state = state.try_lock().unwrap();

        state.toolbar.focused = focused;
    }

    fn is_toolbar_visible(&self) -> bool {
        let state = self.state::<AppState>();
        let state = state.try_lock().unwrap();

        state.toolbar.visible
    }

    fn set_toolbar_visible(&self, visible: bool) {
        let state = self.state::<AppState>();
        let mut state = state.try_lock().unwrap();

        state.toolbar.visible = visible;
    }
}
