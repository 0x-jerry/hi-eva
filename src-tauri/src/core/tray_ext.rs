use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    Manager, Result,
};

use super::{MyApp, MyAppWindowExt, MyUpdaterExt, MAIN_WINDOW_LABEL};

pub trait AppTrayExt {
    fn create_tray(&self) -> Result<TrayIcon>;
}

const MENU_QUIT: &str = "quit";
const MENU_CHECK_UPDATE: &str = "check-update";
const MENU_OPEN_STATISTIC: &str = "open-statistic";

impl AppTrayExt for MyApp {
    fn create_tray(&self) -> Result<TrayIcon> {
        let app = self.app();

        let m_quit = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
        let m_check_update =
            MenuItem::with_id(app, MENU_CHECK_UPDATE, "Check Update", true, None::<&str>)?;
        let m_open_static =
            MenuItem::with_id(app, MENU_OPEN_STATISTIC, "Static Page", true, None::<&str>)?;

        let menu = Menu::with_items(
            app,
            &[
                &m_open_static,
                &PredefinedMenuItem::separator(app)?,
                &m_check_update,
                &m_quit,
            ],
        )?;

        let tray = TrayIconBuilder::new()
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
            _ => {}
        });

        Ok(tray)
    }
}
