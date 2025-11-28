use anyhow::Result;
use rs_utils::macros::chain_from;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfigV1 {
    pub version: i32,
    pub proxy: String,
    pub listen_clipboard: bool,
    pub enabled: bool,
}

impl From<Value> for AppBasicConfigV1 {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfigV2 {
    pub version: i32,
    pub proxy: String,

    pub enable_auto_trigger: bool,
    pub enable_listen_clipboard: bool,
    pub enable_global_shortcut: bool,
    pub global_shortcut: String,
}

impl From<AppBasicConfigV1> for AppBasicConfigV2 {
    fn from(value: AppBasicConfigV1) -> Self {
        AppBasicConfigV2 {
            version: 2,
            proxy: value.proxy,
            enable_auto_trigger: value.enabled,
            enable_listen_clipboard: value.listen_clipboard,
            enable_global_shortcut: false,
            global_shortcut: "".into(),
        }
    }
}

pub type AppBasicConfig = AppBasicConfigV2;

const CONFIGURATION_FILE_NAME: &str = "config.json";
const CONFIGURATION_KEY: &str = "data";

pub fn load(app: &AppHandle) -> Result<AppBasicConfig> {
    let config = app.store(CONFIGURATION_FILE_NAME)?;

    let value = config.get(CONFIGURATION_KEY).unwrap_or_default();

    let version = value
        .get("version")
        .map(|v| v.as_i64())
        .flatten()
        .unwrap_or_default();

    let value: AppBasicConfig = chain_from!(value, AppBasicConfigV1, AppBasicConfigV2);

    if version != value.version as i64 {
        save(app, &value)?;
    }

    Ok(value)
}

pub fn save(app: &AppHandle, value: &AppBasicConfig) -> Result<()> {
    let config = app.store(CONFIGURATION_FILE_NAME)?;

    let data = serde_json::to_value(value)?;
    config.set(CONFIGURATION_KEY, data);

    config.save()?;

    log::info!("configuration saved");

    Ok(())
}
