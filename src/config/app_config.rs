use crate::enums::{
    monitor_type_enum::MonitorTypeEnum, supported_language_enum::SupportedLanguageEnum,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GeneralConfig {
    pub minimized_window_on_startup: bool,
    pub run_on_startup: bool,
    pub language: SupportedLanguageEnum,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimingConfig {
    pub tray_error_retry_delay_ms: u64,
    pub ui_repaint_interval: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RefreshConfig {
    pub default_refresh_seconds: u64,
    pub min_refresh_seconds: u64,
    pub max_refresh_seconds: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WindowConfig {
    pub settings_width: f32,
    pub settings_height: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub active_monitors: HashSet<MonitorTypeEnum>,
    pub general: GeneralConfig,
    pub refresh: RefreshConfig,
    pub timing: TimingConfig,
    pub window: WindowConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config_builder = config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .add_source(config::Environment::with_prefix("APP"));

        config_builder.build()?.try_deserialize()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write("config.toml", toml_string)?;
        Ok(())
    }
}
