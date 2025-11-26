use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result, Wry,
};

use crate::core::ConfigurationExt;

use super::{MyApp, MyAppWindowExt, MyUpdaterExt, MAIN_WINDOW_LABEL};

pub trait AppTrayExt {
    fn create_tray(&self) -> Result<TrayIcon>;
}

const MENU_QUIT: &str = "quit";
const MENU_CHECK_UPDATE: &str = "check-update";
const MENU_OPEN_STATISTIC: &str = "open-statistic";
const MENU_ENABLED_AUTO_TRIGGER: &str = "enabled";
const MENU_LISTEN_CLIPBOARD_ENABLED: &str = "m_clipboard_enabled";

const TRAY_NAME: &str = "main";

impl AppTrayExt for MyApp {
    fn create_tray(&self) -> Result<TrayIcon> {
        let app = self.app();

        let menu = build_tray_menu(app)?;

        let tray = TrayIconBuilder::with_id(TRAY_NAME)
            .menu(&menu)
            .show_menu_on_left_click(false)
            .icon(self.default_window_icon().unwrap().clone())
            .build(app)?;

        tray.on_tray_icon_event(move |icon, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } => {
                log::info!("tray icon left click");

                icon.app_handle()
                    .state::<MyApp>()
                    .open_and_focus(MAIN_WINDOW_LABEL);
            }
            _ => {}
        });

        tray.on_menu_event(move |app, event| match event.id.as_ref() {
            MENU_QUIT => {
                app.exit(0);
            }
            MENU_CHECK_UPDATE => {
                app.state::<MyApp>().check_update();
            }
            MENU_OPEN_STATISTIC => {
                app.state::<MyApp>().get_statistic_window();
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

                let my_app = app.state::<MyApp>();
                my_app.apply_clipboard_listener();

                update_tray_menu(app);
            }
            _ => {}
        });

        Ok(tray)
    }
}

pub fn update_tray_menu(app: &AppHandle) {
    app.tray_by_id(TRAY_NAME).map(|tray| {
        let menu = build_tray_menu(app).ok();
        let _ = tray.set_menu(menu);
    });
}

pub fn build_tray_menu(app: &AppHandle) -> Result<Menu<Wry>> {
    let conf = app.get_conf();

    let m_quit = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
    let m_check_update =
        MenuItem::with_id(app, MENU_CHECK_UPDATE, "Check Update", true, None::<&str>)?;
    let m_open_static =
        MenuItem::with_id(app, MENU_OPEN_STATISTIC, "Static Page", true, None::<&str>)?;

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
            &m_open_static,
            &PredefinedMenuItem::separator(app)?,
            &m_check_update,
            &m_quit,
        ],
    )?;

    Ok(menu)
}
