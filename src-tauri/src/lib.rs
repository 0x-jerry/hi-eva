use core::{MyApp, MyAppWindowExt, MAIN_WINDOW_LABEL};

use tauri::{Manager, RunEvent};

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

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_selected_text,
            commands::open_chat
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            let my_app = MyApp::new(app_handle);
            my_app.init();

            app.manage(my_app);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log::info!("single instance");
            let app = app.state::<MyApp>();
            app.open_and_focus(MAIN_WINDOW_LABEL);
        }));

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
