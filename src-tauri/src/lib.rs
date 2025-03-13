use core::{MyApp, MyAppWindowExt, MAIN_WINDOW_LABEL};

use tauri::{ActivationPolicy, Manager, RunEvent};

mod commands;
mod core;
mod plugins;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(dev)]
    {
        dotenv::from_filename(".env.development").expect("load env failed");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    env_logger::init();

    let app = tauri::Builder::default();

    #[cfg(unix)]
    let app = app.plugin(tauri_nspanel::init());

    let app = app
        .invoke_handler(tauri::generate_handler![
            commands::get_selected_text,
            commands::open_chat,
            commands::apply_appearance
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log::info!("single instance");
            let app = app.state::<MyApp>();
            app.open_and_focus(MAIN_WINDOW_LABEL);
        }))
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);

            let app_handle = app.handle().clone();

            let my_app = MyApp::new(app_handle);
            my_app.init();

            app.manage(my_app);

            Ok(())
        });

    let app = app
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_handle, event| match event {
        RunEvent::Exit => {
            log::info!("app exited");
        }
        _ => (),
    });
}
