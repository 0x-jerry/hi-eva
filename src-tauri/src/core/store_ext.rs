use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

use super::MyApp;

pub trait AppStoreExt {
    fn get_basic_config(&self) -> AppBasicConfig;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfig {
    pub version: i32,
    pub proxy: String,
    pub listen_clipboard: bool,
}

impl Default for AppBasicConfig {
    fn default() -> Self {
        AppBasicConfig {
            version: 1,
            proxy: "".to_string(),
            listen_clipboard: true,
        }
    }
}

impl AppStoreExt for MyApp {
    fn get_basic_config(&self) -> AppBasicConfig {
        let config = self.store("config.json").unwrap();

        let value = config.get("data").map(|x| {
            let value = serde_json::from_value::<AppBasicConfig>(x).unwrap();

            value
        });

        value.unwrap_or_default()
    }
}
