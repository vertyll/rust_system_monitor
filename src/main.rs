#![windows_subsystem = "windows"]

mod app;
mod config;
mod enums;
mod error;
mod i18n;
mod monitor;
mod tray;
mod ui;

use app::App;
use auto_launch::AutoLaunch;
use eframe::{NativeOptions, egui};
use i18n::i18n_manager::I18nManager;
use std::sync::{Arc, Mutex};

use crate::config::app_config::AppConfig;

// fn get_system_language() -> SupportedLanguageEnum {
//     let system_locale = Locale::current();
//     if system_locale.to_string().starts_with("pl") {
//         return SupportedLanguageEnum::Polish;
//     }
//     SupportedLanguageEnum::English
// }

fn main() -> std::result::Result<(), eframe::Error> {
    let app_config = AppConfig::new().expect("Failed to load config.toml");
    let i18n_manager = Arc::new(Mutex::new(I18nManager::new(app_config.general.language)));
    let app_name = &app_config.app_name;
    let app_path = std::env::current_exe().unwrap().display().to_string();
    let auto_launch = AutoLaunch::new(app_name, &app_path, &[] as &[&str]);

    let (width, height) = (
        app_config.window.settings_width,
        app_config.window.settings_height,
    );
    // let window_title = {
    //     let i18n_guard = i18n_manager.lock().unwrap();
    //     i18n_guard.get_message("name")
    // };

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(app_name.to_string())
            .with_inner_size([width, height])
            .with_resizable(true)
            .with_visible(true)
            .with_taskbar(false),
        ..Default::default()
    };

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| {
            let app = App::new(cc, app_config.clone(), i18n_manager, auto_launch)
                .expect("Failed to create App");
            Ok(Box::new(app))
        }),
    )
}
