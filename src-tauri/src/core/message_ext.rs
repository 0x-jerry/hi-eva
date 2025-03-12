use tauri::{Emitter, EventTarget, LogicalPosition, Manager, PhysicalPosition};

use super::{AppState, MyApp, MyAppWindowExt, CHAT_WINDDOW_LABEL, TOOLBAR_WINDDOW_LABEL};

pub trait AppMessageExt {
    fn hide_toolbar(&self);
    fn open_chat(&self, prompt_id: String);
    fn open_toolbar(&self, selected_text: String);
}

impl AppMessageExt for MyApp {
    fn open_chat(&self, prompt_id: String) {
        let win = self.get_chat_window();

        win.emit_to(
            EventTarget::labeled(CHAT_WINDDOW_LABEL),
            "show-chat",
            prompt_id,
        )
        .unwrap();

        let pos = cacl_window_position(self);
        win.set_position(pos).unwrap();

        win.show().unwrap();
        win.set_focus().unwrap();
    }

    fn hide_toolbar(&self) {
        let win = self.get_toolbar_window();

        win.emit_to(EventTarget::labeled(TOOLBAR_WINDDOW_LABEL), "hide", ())
            .unwrap();
    }

    fn open_toolbar(&self, selected_text: String) {
        if selected_text.len() <= 0 {
            return;
        }

        log::info!("selected text is {}", selected_text);

        {
            // update state
            let state = self.state::<AppState>();
            let mut state = state.lock().unwrap();
            state.selected_text = selected_text.to_string().clone();
        }

        let win = self.get_toolbar_window();

        let pos = cacl_window_position(&self);

        win.set_position(pos).unwrap();
        win.set_always_on_top(true).unwrap();

        self.emit_to(EventTarget::labeled(TOOLBAR_WINDDOW_LABEL), "show", ())
            .unwrap();
    }
}

fn cacl_window_position(app: &MyApp) -> PhysicalPosition<f64> {
    let offset_pos: PhysicalPosition<f64> =
        LogicalPosition::new(10.0, 5.0).to_physical(app.scale_factor());

    let mouse_pos = app.cursor_position().unwrap();
    // todo, calc windows position
    let pos = PhysicalPosition::new(mouse_pos.x + offset_pos.x, mouse_pos.y + offset_pos.y);
    pos
}
