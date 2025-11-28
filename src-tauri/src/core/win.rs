use serde::Serialize;
use tauri::{
    AppHandle, Emitter, EventTarget, Manager, PhysicalPosition, WebviewWindow, WebviewWindowBuilder,
};

use crate::core::{constant::event_name, AppState};

use super::{utils::calc_window_position, VerticalMoveDir};

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const TOOLBAR_WINDOW_LABEL: &str = "toolbar";
pub const CHAT_WINDOW_LABEL: &str = "chat";
pub const TOOLBAR_HIDDEN_LOWEST_Y_POS: i32 = -9999;

pub fn get_main_window(app: &AppHandle) -> WebviewWindow {
    if let Some(win) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        return win;
    }

    let win_builder = WebviewWindowBuilder::new(
        app,
        MAIN_WINDOW_LABEL,
        tauri::WebviewUrl::App("#/main".into()),
    )
    .inner_size(800.0, 600.0)
    .center()
    .accept_first_mouse(true)
    .visible_on_all_workspaces(true)
    .visible(true);

    let win = win_builder.build().expect("Create main window failed!");

    let app = app.clone();

    win.on_window_event(move |event| match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            get_main_window(&app).hide().unwrap();
        }
        _ => {}
    });

    log::info!("Create main window");

    return win;
}

pub fn get_toolbar_window(app: &AppHandle) -> WebviewWindow {
    if let Some(win) = app.get_webview_window(TOOLBAR_WINDOW_LABEL) {
        return win;
    }

    let win_builder = WebviewWindowBuilder::new(
        app,
        TOOLBAR_WINDOW_LABEL,
        tauri::WebviewUrl::App("#/toolbar".into()),
    )
    .inner_size(0.0, 0.0)
    .visible_on_all_workspaces(true)
    .accept_first_mouse(true)
    .decorations(false)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .transparent(true)
    .skip_taskbar(true)
    .focused(false)
    .always_on_top(true);

    let win = win_builder.build().expect("Create toolbar window failed!");
    log::info!("Create toolbar window");

    #[cfg(unix)]
    {
        use crate::plugins::MacWindowExt;
        let _ = win
            .to_non_active_panel()
            .expect("convert to non active panel failed");
    }

    win.show().unwrap();

    hide_toolbar_win(app);

    return win;
}

pub fn get_chat_window(app: &AppHandle) -> WebviewWindow {
    if let Some(win) = app.get_webview_window(CHAT_WINDOW_LABEL) {
        return win;
    }

    let win_builder = WebviewWindowBuilder::new(
        app,
        CHAT_WINDOW_LABEL,
        tauri::WebviewUrl::App("#/chat".into()),
    )
    .inner_size(400.0, 600.0)
    .focused(false)
    .decorations(false)
    .resizable(false)
    .visible(false)
    .transparent(true)
    .accept_first_mouse(true)
    .visible_on_all_workspaces(true)
    .skip_taskbar(true)
    .minimizable(false)
    .maximizable(false)
    .always_on_top(false);

    let win = win_builder.build().expect("Create toolbar window failed!");

    log::info!("Create chat window");

    return win;
}

pub fn init_windows(app: &AppHandle) {
    get_main_window(app);
    get_toolbar_window(app);
    get_chat_window(app);
}

/// Move toolbar to invisible area
pub fn hide_toolbar_win(app: &AppHandle) {
    let win = get_toolbar_window(app);

    let old_pos = win.outer_position().unwrap();

    let pos = PhysicalPosition::new(old_pos.x, TOOLBAR_HIDDEN_LOWEST_Y_POS - 100);

    win.set_position(pos).unwrap();

    #[cfg(unix)]
    {
        use crate::plugins::MacWindowExt;
        win.ns_resign_focus().unwrap();
    }

    let state = app.state::<AppState>();
    state.set_toolbar_focused(false);
}

pub fn show_toolbar_win(app: &AppHandle, dir: Option<VerticalMoveDir>) {
    let win = get_toolbar_window(app);

    win.emit_to(
        EventTarget::labeled(TOOLBAR_WINDOW_LABEL),
        event_name::TOOLBAR_SHOW,
        (),
    )
    .unwrap();

    let pos = calc_window_position(&win, dir);

    win.set_position(pos).unwrap();
}

pub fn open_chat_win(app: &AppHandle, prompt_id: String) {
    let win = get_chat_window(app);

    let state = app.state::<AppState>();

    if !state.is_chat_panel_pinned() {
        let pos = calc_window_position(&win, None);
        win.set_position(pos).unwrap();
    }

    #[derive(Debug, Serialize, Clone)]
    struct OpenChatPayload {
        prompt_id: String,
        selected_text: String,
    }

    win.emit_to(
        EventTarget::labeled(CHAT_WINDOW_LABEL),
        event_name::CHAT_SHOW,
        OpenChatPayload {
            prompt_id,
            selected_text: state.get_selected_text(),
        },
    )
    .unwrap();

    win.show().unwrap();
    win.set_focus().unwrap();
}

pub fn hide_chat_win(app: &AppHandle) {
    let win = get_chat_window(app);

    win.emit_to(
        EventTarget::labeled(CHAT_WINDOW_LABEL),
        event_name::CHAT_HIDE,
        (),
    )
    .unwrap();
}
