use crate::{
    config::app_config::AppConfig,
    enums::{monitor_type_enum::MonitorTypeEnum, supported_language_enum::SupportedLanguageEnum},
    i18n::i18n_manager::I18nManager,
    ui::components,
};
use eframe::egui;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

pub fn draw_ui(
    ui: &mut egui::Ui,
    app_config: Arc<Mutex<AppConfig>>,
    i18n: Arc<Mutex<I18nManager>>,
) -> (bool, bool, bool) {
    let mut shutdown_requested = false;
    let mut language_changed = false;
    let mut autostart_setting_changed = false;

    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                let title = i18n.lock().unwrap().get_message("settings-title");
                ui.heading(title);
                ui.add_space(5.0);
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            egui::Grid::new("settings_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    let mut ac = app_config.lock().unwrap();
                    let mut i18n_guard = i18n.lock().unwrap();

                    let label = i18n_guard.get_message("refresh-time-label");
                    ui.label(label);
                    let mut refresh_secs = ac.refresh.default_refresh_seconds;
                    let range = ac.refresh.min_refresh_seconds..=ac.refresh.max_refresh_seconds;
                    if ui
                        .add(egui::Slider::new(&mut refresh_secs, range).text("s"))
                        .changed()
                    {
                        ac.refresh.default_refresh_seconds = refresh_secs;
                    }
                    ui.end_row();

                    let lang_label = i18n_guard.get_message("language-label");
                    ui.label(lang_label);
                    let mut current_lang = ac.general.language;
                    egui::ComboBox::from_id_salt("language_combo_box")
                        .selected_text(current_lang.name())
                        .show_ui(ui, |ui| {
                            for lang in SupportedLanguageEnum::iter() {
                                if ui
                                    .selectable_value(&mut current_lang, lang, lang.name())
                                    .changed()
                                {
                                    ac.general.language = current_lang;
                                    *i18n_guard = I18nManager::new(current_lang);
                                    language_changed = true;
                                }
                            }
                        });
                    ui.end_row();

                    let autostart_label = i18n_guard.get_message("run-on-startup-label");
                    ui.label(autostart_label);

                    if ui
                        .add(components::toggle_switch_component::toggle(
                            &mut ac.general.run_on_startup,
                        ))
                        .changed()
                    {
                        autostart_setting_changed = true;
                    }
                    ui.end_row();

                    let minimized_label = i18n_guard.get_message("minimized-on-startup-label");
                    ui.label(minimized_label);

                    ui.add(components::toggle_switch_component::toggle(
                        &mut ac.general.minimized_window_on_startup,
                    ));
                    ui.end_row();

                    for monitor_type in MonitorTypeEnum::iter() {
                        let label_key = monitor_type.icon_label_key();
                        let label = format!(
                            "{} {}",
                            i18n_guard.get_message("monitor-label-prefix"),
                            i18n_guard.get_message(label_key)
                        );
                        ui.label(label);
                        let mut is_active = ac.active_monitors.contains(&monitor_type);
                        if ui
                            .add(components::toggle_switch_component::toggle(&mut is_active))
                            .changed()
                        {
                            if is_active {
                                ac.active_monitors.insert(monitor_type);
                            } else {
                                ac.active_monitors.remove(&monitor_type);
                            }
                        }
                        ui.end_row();
                    }
                });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            let label = i18n.lock().unwrap().get_message("shutdown-button-label");
            if ui.button(label).clicked() {
                shutdown_requested = true;
            }
        });
    });

    (
        shutdown_requested,
        language_changed,
        autostart_setting_changed,
    )
}
