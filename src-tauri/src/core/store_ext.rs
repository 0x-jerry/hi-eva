use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfig {
    pub version: i32,
    pub proxy: String,
    pub listen_clipboard: bool,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl Default for AppBasicConfig {
    fn default() -> Self {
        AppBasicConfig {
            version: 1,
            proxy: "".to_string(),
            listen_clipboard: true,
            enabled: true,
        }
    }
}

impl AppBasicConfig {
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
