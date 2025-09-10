use crate::{enums::monitor_type_enum::MonitorTypeEnum, tray::tray::TrayItem};
use tray_icon::TrayIcon;

pub struct RamUsageTrayItem {
    pub icon: TrayIcon,
}

impl TrayItem for RamUsageTrayItem {
    fn get_type(&self) -> MonitorTypeEnum {
        MonitorTypeEnum::RamUsage
    }

    fn icon(&self) -> &TrayIcon {
        &self.icon
    }
}
