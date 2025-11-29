use anyhow::Result;
use rs_utils::{
    macros::{migration, Versioned},
    migration::Versioned,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smart_default::SmartDefault;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, SmartDefault, Clone, Versioned)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfigV1 {
    #[default = 1]
    pub version: u32,
    pub proxy: String,
    pub listen_clipboard: bool,
    pub enabled: bool,
}

impl From<Value> for AppBasicConfigV1 {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, Clone, Versioned)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfigV2 {
    #[default = 2]
    pub version: u32,
    pub proxy: String,

    pub enable_auto_trigger: bool,
    pub enable_listen_clipboard: bool,
    pub enable_global_shortcut: bool,
    pub global_shortcut: String,
}

impl From<AppBasicConfigV1> for AppBasicConfigV2 {
    fn from(value: AppBasicConfigV1) -> Self {
        AppBasicConfigV2 {
            proxy: value.proxy,
            enable_auto_trigger: value.enabled,
            enable_listen_clipboard: value.listen_clipboard,
            enable_global_shortcut: false,
            global_shortcut: "".into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, Clone, Versioned)]
#[serde(rename_all = "camelCase")]
pub struct AppBasicConfigV3 {
    #[default = 3]
    pub version: u32,
    pub proxy: String,

    pub enable_auto_trigger: bool,
    pub enable_listen_clipboard: bool,
    pub enable_global_shortcut: bool,
    pub global_shortcut: String,

    pub autostart: bool,
}

impl From<AppBasicConfigV2> for AppBasicConfigV3 {
    fn from(value: AppBasicConfigV2) -> Self {
        AppBasicConfigV3 {
            proxy: value.proxy,
            enable_auto_trigger: value.enable_auto_trigger,
            enable_listen_clipboard: value.enable_listen_clipboard,
            enable_global_shortcut: value.enable_global_shortcut,
            global_shortcut: value.global_shortcut,
            ..Default::default()
        }
    }
}

pub type AppBasicConfig = AppBasicConfigV3;

const CONFIGURATION_FILE_NAME: &str = "config.json";
const CONFIGURATION_KEY: &str = "data";

pub fn load(app: &AppHandle) -> Result<AppBasicConfig> {
    let config = app.store(CONFIGURATION_FILE_NAME)?;

    let value = config.get(CONFIGURATION_KEY).unwrap_or_default();

    let version = value
        .get("version")
        .map(|v| v.as_u64())
        .flatten()
        .unwrap_or_default() as u32;

    let value: AppBasicConfig =
        migration!(value, AppBasicConfigV1, AppBasicConfigV2, AppBasicConfigV3);

    if version != value.version {
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
