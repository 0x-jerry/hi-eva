use core::MyApp;

use tauri::Manager;

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

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
