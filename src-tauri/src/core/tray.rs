use anyhow::Result;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Wry,
};

use crate::core::{apply_watch_clipboard, check_update, ConfigurationExt, MAIN_WINDOW_LABEL};

use super::AppWindowExt;

const MENU_QUIT: &str = "quit";
const MENU_CHECK_UPDATE: &str = "check-update";
const MENU_ENABLED_AUTO_TRIGGER: &str = "enabled";
const MENU_LISTEN_CLIPBOARD_ENABLED: &str = "m_clipboard_enabled";

const TRAY_NAME: &str = "main";

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon> {
    let menu = build_tray_menu(app)?;

    let tray = TrayIconBuilder::with_id(TRAY_NAME)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    tray.on_tray_icon_event(move |icon, event| match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            ..
        } => {
            log::info!("tray icon left click");

            icon.app_handle().open_and_focus(MAIN_WINDOW_LABEL);
        }
        _ => {}
    });

    tray.on_menu_event(move |app, event| match event.id.as_ref() {
        MENU_QUIT => {
            app.exit(0);
        }
        MENU_CHECK_UPDATE => {
            check_update(app);
        }
        MENU_ENABLED_AUTO_TRIGGER => {
            let mut conf = app.get_conf();
            conf.enable_auto_trigger = !conf.enable_auto_trigger;
            app.save_conf(&conf);

            update_tray_menu(app);
        }
        MENU_LISTEN_CLIPBOARD_ENABLED => {
            let mut conf = app.get_conf();
            conf.enable_listen_clipboard = !conf.enable_listen_clipboard;
            app.save_conf(&conf);

            update_tray_menu(app);

            apply_watch_clipboard(app).expect("Apply clipboard listener failed!");
        }
        _ => {}
    });

    Ok(tray)
}

pub fn update_tray_menu(app: &AppHandle) {
    app.tray_by_id(TRAY_NAME).map(|tray| {
        let menu = build_tray_menu(app).ok();
        let _ = tray.set_menu(menu);
    });
}

fn build_tray_menu(app: &AppHandle) -> Result<Menu<Wry>> {
    let conf = app.get_conf();

    let m_quit = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
    let m_check_update =
        MenuItem::with_id(app, MENU_CHECK_UPDATE, "Check Update", true, None::<&str>)?;

    let m_enable_auto_trigger = CheckMenuItem::with_id(
        app,
        MENU_ENABLED_AUTO_TRIGGER,
        "Auto Trigger",
        true,
        conf.enable_auto_trigger,
        None::<&str>,
    )?;

    let m_listen_clipboard = CheckMenuItem::with_id(
        app,
        MENU_LISTEN_CLIPBOARD_ENABLED,
        "Listen Clipboard",
        true,
        conf.enable_listen_clipboard,
        None::<&str>,
    )?;

    let menu = Menu::with_items(
        app,
        &[
            &m_enable_auto_trigger,
            &m_listen_clipboard,
            &PredefinedMenuItem::separator(app)?,
            &m_check_update,
            &m_quit,
        ],
    )?;

    Ok(menu)
}
