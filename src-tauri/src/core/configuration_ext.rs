use crate::core::{configuration, constant::event_name, AppBasicConfig};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

pub type ConfigState = Mutex<AppBasicConfig>;

pub fn init_manager(app: AppHandle) {
    let conf = app.reload_conf();

    app.manage(Mutex::new(conf));
}

pub trait ConfigurationExt {
    fn get_conf(&self) -> AppBasicConfig;

    fn reload_conf(&self) -> AppBasicConfig;

    fn save_conf(&self, new_conf: &AppBasicConfig);
}

impl ConfigurationExt for AppHandle {
    fn get_conf(&self) -> AppBasicConfig {
        let binding = self.state::<ConfigState>();

        return binding.lock().unwrap().clone();
    }

    fn reload_conf(&self) -> AppBasicConfig {
        let conf = configuration::load(self).expect("Load configuration failed!");

        return conf;
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
