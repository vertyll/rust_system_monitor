use crate::{enums::monitor_type_enum::MonitorTypeEnum, tray::tray::TrayItem};
use tray_icon::TrayIcon;

pub struct CpuUsageTrayItem {
    pub icon: TrayIcon,
}

impl TrayItem for CpuUsageTrayItem {
    fn get_type(&self) -> MonitorTypeEnum {
        MonitorTypeEnum::CpuUsage
    }

    fn icon(&self) -> &TrayIcon {
        &self.icon
    }
}
