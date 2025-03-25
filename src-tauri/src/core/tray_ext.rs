use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    Manager, Result,
};

use super::{MyApp, MyAppWindowExt, MyUpdaterExt, MAIN_WINDOW_LABEL};

pub trait AppTrayExt {
    fn create_tray(&self) -> Result<TrayIcon>;
}

impl AppTrayExt for MyApp {
    fn create_tray(&self) -> Result<TrayIcon> {
        let app = self.app();

        let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        let check_update_i =
            MenuItem::with_id(app, "check-update", "Check Update", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&quit_i, &check_update_i])?;

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
            "quit" => {
                app.exit(0);
            }
            "check-update" => {
                app.state::<MyApp>().check_update();
            }
            _ => {}
        });

        Ok(tray)
    }
}
