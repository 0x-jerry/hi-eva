use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfig {
    pub version: i32,
    pub proxy: String,

    #[serde(default = "default_true")]
    pub enable_auto_trigger: bool,
    #[serde(default = "default_true")]
    pub enable_listen_clipboard: bool,
    #[serde(default = "default_true")]
    pub enable_global_shortcut: bool,
    #[serde(default = "default_string")]
    pub global_shortcut: String,
}

fn default_true() -> bool {
    true
}

fn default_string() -> String {
    String::new()
}

impl Default for AppBasicConfig {
    fn default() -> Self {
        AppBasicConfig {
            version: 1,
            proxy: "".to_string(),
            enable_listen_clipboard: true,
            enable_auto_trigger: true,
            enable_global_shortcut: true,
            global_shortcut: "".into(),
        }
    }
}

impl AppBasicConfig {
    pub fn init(app: &AppHandle) -> AppBasicConfig {
        let config = app.store("config.json").expect("get store failed!");

        let value = config
            .get("data")
            .map(|x| {
                let value = serde_json::from_value::<AppBasicConfig>(x);

                value.unwrap_or_default()
            })
            .unwrap_or_default();

        value.save(app);

        value
    }

    pub fn load<R: Runtime>(app: &AppHandle<R>) -> AppBasicConfig {
        let config = app.store("config.json").expect("get store failed!");

        let value = config.get("data").map(|x| {
            let value = serde_json::from_value::<AppBasicConfig>(x);

            value.unwrap_or_default()
        });

        value.unwrap_or_default()
    }

    pub fn save(&self, app: &AppHandle) {
        let store_config = app.store("config.json").expect("get store failed!");

        let conf_value = serde_json::to_value(self).expect("convert to json failed!");

        store_config.set("data", conf_value.clone());

        app.webview_windows().iter().for_each(|(_name, win)| {
            #[derive(Serialize, Clone)]
            #[serde(rename_all = "camelCase")]
            struct SyncEventPayload {
                file_name: String,
                key: String,
                data: serde_json::Value,
                id: String,
            }

            let payload = SyncEventPayload {
                file_name: "config.json".to_string(),
                key: "data".to_string(),
                data: conf_value.clone(),
                id: nanoid!(),
            };

            win.emit("store-changed", payload).unwrap();
        });
    }
}
