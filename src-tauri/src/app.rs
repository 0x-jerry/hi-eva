use clipboard_rs::{Clipboard, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, EventTarget, LogicalPosition, Manager, WebviewWindow, WebviewWindowBuilder,
};
use text_selection::{ListenResult, TextSelectionHandler};

#[derive(Clone)]
pub struct MyApp {
    app: AppHandle,
}

impl MyApp {
    pub fn new(app: AppHandle) -> MyApp {
        MyApp { app }
    }

    pub fn init(&self) {
        let _ = self.create_tray();

        let _ = self.get_or_create_main_window();
        let _ = self.get_or_create_chat_window();
        let _ = self.get_or_create_toolbar_window();

        let app_cloned = self.clone();
        tauri::async_runtime::spawn_blocking(move || {
            //
            let mut watcher =
                ClipboardWatcherContext::<MyApp>::new().expect("Init clipboard watcher");
            watcher.add_handler(app_cloned);
            watcher.start_watch();
        });

        let app_cloned = self.clone();
        #[cfg(windows)]
        tauri::async_runtime::spawn_blocking(move || {
            let _ = text_selection::listen(app_cloned);
        });

        #[cfg(unix)]
        let _ = text_selection::listen(app_cloned);
    }

    pub fn get_selected_text(&self) -> Option<String> {
        if let Ok(selected) = text_selection::get_selected_text() {
            return Some(selected);
        }

        return Self::get_selected_text_from_clipboard();
    }

    fn get_selected_text_from_clipboard() -> Option<String> {
        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            let selected_text = selected_text.trim();
            return Some(selected_text.to_string());
        }

        return None;
    }

    fn get_or_create_main_window(&self) -> WebviewWindow {
        let win_label = "main";

        if let Some(win) = self.app.get_webview_window(&win_label) {
            return win;
        }

        let win_builder =
            WebviewWindowBuilder::new(&self.app, win_label, tauri::WebviewUrl::App("#/".into()))
                .inner_size(800.0, 600.0)
                .center()
                .visible(true);

        let win = win_builder.build().expect("Create main window failed!");

        return win;
    }

    fn create_tray(&self) -> tauri::Result<TrayIcon> {
        let app = &self.app;
        let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&quit_i])?;

        let tray = TrayIconBuilder::new()
            .menu(&menu)
            .show_menu_on_left_click(false)
            .build(app)?;

        let my = self.clone();
        tray.on_tray_icon_event(move |_icon, event| match event {
            TrayIconEvent::Click {
                id: _,
                position: _,
                rect: _,
                button: MouseButton::Left,
                button_state: _,
            } => {
                my.open_main_win();
            }
            _ => {}
        });

        let my = self.clone();
        tray.on_menu_event(move |_app, event| match event.id.as_ref() {
            "quit" => {
                my.app.exit(0);
            }
            _ => {}
        });

        Ok(tray)
    }

    fn open_main_win(&self) {
        let main_win = self.get_or_create_main_window();
        main_win.show().unwrap();
        main_win.set_focus().unwrap();
    }

    fn get_or_create_toolbar_window(&self) -> WebviewWindow {
        let win_label = "toolbar";

        if let Some(win) = self.app.get_webview_window(&win_label) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            &self.app,
            win_label,
            tauri::WebviewUrl::App("#/toolbar".into()),
        )
        .inner_size(300.0, 60.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .visible(false)
        .skip_taskbar(true)
        .always_on_top(true);

        let win = win_builder.build().expect("Create toolbar window failed!");

        return win;
    }

    pub fn get_or_create_chat_window(&self) -> WebviewWindow {
        let win_label = "chat";

        if let Some(win) = self.app.get_webview_window(&win_label) {
            return win;
        }

        let win_builder = WebviewWindowBuilder::new(
            &self.app,
            win_label,
            tauri::WebviewUrl::App("#/chat".into()),
        )
        .inner_size(400.0, 600.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .visible(false)
        .skip_taskbar(true)
        .always_on_top(false);

        let win = win_builder.build().expect("Create toolbar window failed!");

        return win;
    }
}

impl ClipboardHandler for MyApp {
    fn on_clipboard_change(&mut self) {
        log::info!("clipboard changed");

        let clipboard = clipboard_rs::ClipboardContext::new().unwrap();

        if let Ok(selected_text) = clipboard.get_text() {
            log::info!("clipboard text: {:?}", selected_text);

            if selected_text.trim().len() > 0 {
                self.on_selection_change(ListenResult {
                    selected_text,
                    mouse_position: text_selection::get_mouse_pos(),
                });
            }
        }
    }
}

impl TextSelectionHandler for MyApp {
    fn on_selection_change(&self, result: ListenResult) {
        if result.selected_text.trim().len() <= 0 {
            return;
        }

        println!("selected: {:?}", result);

        let win = self.get_or_create_toolbar_window();

        let offset_pos = (8.0, 4.0);

        // todo, calc windows position
        let pos = LogicalPosition::new(
            result.mouse_position.0 + offset_pos.0,
            result.mouse_position.1 + offset_pos.1,
        );

        win.set_position(pos).unwrap();

        self.app
            .emit_to(EventTarget::labeled("toolbar"), "show", ())
            .expect("Notify toolbar window");
    }
}
