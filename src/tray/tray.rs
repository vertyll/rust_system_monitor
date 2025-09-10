use crate::enums::{monitor_type_enum::MonitorTypeEnum, tray_menu_event_enum::TrayMenuEventEnum};
use crate::error::app_error::Result;
use crate::i18n::i18n_manager::I18nManager;
use crate::tray::cpu_usage_tray_item::CpuUsageTrayItem;
use crate::tray::ram_usage_tray_item::RamUsageTrayItem;
use crossbeam_channel::Receiver;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
};

pub trait Tray {
    fn update(
        &self,
        active_monitors: &std::collections::HashSet<MonitorTypeEnum>,
        i18n: &I18nManager,
        stats: &[(MonitorTypeEnum, f32)],
    ) -> Result<()>;
}

pub trait TrayItem {
    fn get_type(&self) -> MonitorTypeEnum;
    fn icon(&self) -> &TrayIcon;
}

pub struct SystemTray {
    items: Vec<Box<dyn TrayItem>>,
}

impl SystemTray {
    pub fn new(
        i18n: &I18nManager,
    ) -> Result<(
        Self,
        Receiver<MenuEvent>,
        HashMap<MenuId, TrayMenuEventEnum>,
    )> {
        let mut id_map = HashMap::new();
        let menu = Menu::new();
        let settings_item = MenuItem::new(i18n.get_message("tray-settings-item"), true, None);
        let quit_item = MenuItem::new(i18n.get_message("tray-shutdown-item"), true, None);
        id_map.insert(settings_item.id().clone(), TrayMenuEventEnum::Settings);
        id_map.insert(quit_item.id().clone(), TrayMenuEventEnum::Quit);
        menu.append_items(&[&settings_item, &PredefinedMenuItem::separator(), &quit_item])?;

        let mut items: Vec<Box<dyn TrayItem>> = Vec::new();

        for monitor_type in MonitorTypeEnum::iter() {
            let tooltip_key = monitor_type.tray_tooltip_key();
            let tooltip = i18n.get_message(tooltip_key);

            let icon = TrayIconBuilder::new()
                .with_menu(Box::new(menu.clone()))
                .with_tooltip(tooltip)
                .build()?;
            icon.set_visible(false)?;

            let tray_item: Box<dyn TrayItem> = match monitor_type {
                MonitorTypeEnum::CpuUsage => Box::new(CpuUsageTrayItem { icon }),
                MonitorTypeEnum::RamUsage => Box::new(RamUsageTrayItem { icon }),
            };
            items.push(tray_item);
        }

        Ok((Self { items }, MenuEvent::receiver().clone(), id_map))
    }
}

impl Tray for SystemTray {
    fn update(
        &self,
        active_monitors: &std::collections::HashSet<MonitorTypeEnum>,
        i18n: &I18nManager,
        stats: &[(MonitorTypeEnum, f32)],
    ) -> Result<()> {
        let stats_map: HashMap<MonitorTypeEnum, f32> = stats.iter().cloned().collect();

        for item in &self.items {
            let monitor_type = item.get_type();
            let is_visible = active_monitors.contains(&monitor_type);

            item.icon().set_visible(is_visible)?;

            if is_visible {
                if let Some(value) = stats_map.get(&monitor_type) {
                    let unit = monitor_type.unit();

                    let label_key = monitor_type.icon_label_key();
                    let label = i18n.get_message(label_key);

                    let rgba = generate_icon_rgba(&label, *value, unit);
                    let icon_img = Icon::from_rgba(rgba, 32, 32)?;
                    item.icon().set_icon(Some(icon_img))?;
                }
            }
        }
        Ok(())
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        self.items.clear();
    }
}

fn generate_icon_rgba(label: &str, value: f32, unit: &str) -> Vec<u8> {
    use ab_glyph::{FontRef, PxScale};
    use image::{Rgba, RgbaImage};
    use imageproc::drawing::draw_text_mut;

    let width = 32;
    let height = 32;
    let mut img = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 0]));
    let font_data = include_bytes!("../../resources/fonts/DejaVuSansMono.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();
    let text_color = Rgba([255, 255, 255, 255]);
    let scale_label = PxScale::from(16.0);
    draw_text_mut(&mut img, text_color, 2, 0, scale_label, &font, label);
    let scale_value = PxScale::from(16.0);
    let value_text = format!("{:.0}{}", value, unit);
    draw_text_mut(&mut img, text_color, 2, 16, scale_value, &font, &value_text);
    img.into_raw()
}
