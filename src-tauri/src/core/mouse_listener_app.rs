use std::ops::Deref;

use anyhow::Result;
use tauri::{AppHandle, Manager};

use crate::{
    core::{
        get_chat_window, get_toolbar_window, hide_chat_win, hide_toolbar_win, show_toolbar_win,
        AppState, ConfigurationExt,
    },
    plugins::MyWebviewWindowExt,
};

use super::{mouse_listener, AppWindowExt, MouseExtTrait, SelectionResult};

struct MouseListenerApp(AppHandle);

impl Deref for MouseListenerApp {
    type Target = AppHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MouseExtTrait for MouseListenerApp {
    fn on_selection_change(&self, result: Option<SelectionResult>) {
        if !self.get_conf().enable_auto_trigger {
            return;
        }

        if let Some(result) = result {
            log::info!("result is {:?}", result);

            if result.rect.text.clone().is_some_and(|x| x.len() > 0) {
                {
                    let state = self.state::<AppState>();
                    let selected_text = result.rect.text.unwrap_or_default();
                    state.set_selected_text(selected_text);
                }

                show_toolbar_win(self, Some(result.mouse_move_dir));
            }
        }
    }

    fn on_mouse_down(&self) {
        let win_toolbar = get_toolbar_window(self);

        if self.is_toolbar_visible() && !win_toolbar.is_cursor_in() {
            hide_toolbar_win(self);
        }

        if get_chat_window(self).is_click_outside() {
            hide_chat_win(self);
        }
    }

    fn on_mouse_move(&self) {
        #[cfg(unix)]
        {
            use crate::plugins::MacWindowExt;

            if !self.is_toolbar_visible() {
                return;
            }

            let win_toolbar = get_toolbar_window(self);

            if win_toolbar.is_cursor_in() {
                win_toolbar.ns_focus().unwrap();
            } else {
                win_toolbar.ns_resign_focus().unwrap();
            }
        }
    }
}

pub fn init_mouse_listener(app: &AppHandle) -> Result<()> {
    mouse_listener::listen(MouseListenerApp(app.clone()))?;

    Ok(())
}
