use core::{AppWindowExt, MAIN_WINDOW_LABEL};

use anyhow::Result;
use tauri::{App, Manager, RunEvent};
use tauri_plugin_autostart::MacosLauncher;

use crate::core::{
    clipboard_listener, configuration_ext, global_shortcut, mouse_listener_app, AppState,
};

mod commands;
mod core;
mod plugins;
mod sql;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(dev)]
    {
        dotenv::from_filename(".env.development").expect("load env failed");
        unsafe  {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }
    env_logger::init();

    let mut context = tauri::generate_context!();

    #[cfg(dev)]
    {
        context.config_mut().identifier = "hi-eva-dev.0x-jerry.com".to_string();
    }

    let app = tauri::Builder::default();

    #[cfg(unix)]
    let app = app.plugin(tauri_nspanel::init());

    let app = app
        .invoke_handler(tauri::generate_handler![
            commands::get_selected_text,
            commands::open_chat,
            commands::apply_appearance,
            commands::set_chat_pinned,
            commands::hide_toolbar_window,
            commands::apply_clipboard_listener,
            commands::open_setting_folder,
            commands::apply_global_shortcut,
            commands::apply_auto_trigger,
            commands::apply_autostart,
            commands::get_configuration,
            commands::save_configuration,
        ])
        .plugin(sql::init_sql())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized", "--autostart"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log::info!("single instance");
            app.open_and_focus(MAIN_WINDOW_LABEL);
        }))
        .setup(|app| {
            setup_app(app)?;

            Ok(())
        });

    let app = app
        .build(context)
        .expect("error while building tauri application");

    app.run(|_handle, event| match event {
        RunEvent::Exit => {
            log::info!("app exited");
        }
        _ => (),
    });
}

fn setup_app(app: &mut App) -> Result<()> {
    #[cfg(unix)]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let app_handle = app.handle();

    configuration_ext::init_manager(app_handle);

    app_handle.manage(AppState::default());

    core::tray::create_tray(app_handle)?;

    core::win::init_windows(app_handle);

    let has_permission = text_selection::check_permissions();

    if has_permission {
        clipboard_listener::apply_watch_clipboard(app_handle)?;
        mouse_listener_app::init_mouse_listener(app_handle)?;
    }

    // Ignore error
    let _ = global_shortcut::apply_global_shortcut(app_handle);

    let app_handle = app_handle.clone();

    if !check_if_start_by_autostart() {
        tauri::async_runtime::spawn(async move {
            std::thread::sleep(std::time::Duration::from_millis(500));
            app_handle.open_and_focus(MAIN_WINDOW_LABEL);
        });
    }

    Ok(())
}

fn check_if_start_by_autostart() -> bool {
    let args = std::env::args().collect::<Vec<_>>();
    args.contains(&"--autostart".into())
}
