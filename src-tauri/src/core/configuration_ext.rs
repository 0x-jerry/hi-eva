use crate::core::{
    configuration::{self, AppBasicConfig},
    constant::event_name,
};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

pub type ConfigState = Mutex<AppBasicConfig>;

pub fn init_manager(app: &AppHandle) {
    let conf = configuration::load(app).expect("Load configuration failed!");

    app.manage(Mutex::new(conf));
}

pub trait ConfigurationExt {
    fn get_conf(&self) -> AppBasicConfig;

    fn save_conf(&self, new_conf: &AppBasicConfig);
}

impl ConfigurationExt for AppHandle {
    fn get_conf(&self) -> AppBasicConfig {
        let binding = self.state::<ConfigState>();

        return binding.lock().unwrap().clone();
    }

    fn save_conf(&self, new_conf: &AppBasicConfig) {
        let binding = self.state::<ConfigState>();
        let mut value = binding.lock().unwrap();
        *value = new_conf.clone();

        // Emit configuration changed event
        self.webview_windows().iter().for_each(|(_name, win)| {
            win.emit(event_name::CONFIGURATION_CHANGED, ()).unwrap();
        });

        configuration::save(self, new_conf).expect("Save configuration failed");
    }
}
