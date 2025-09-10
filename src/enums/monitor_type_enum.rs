use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum MonitorTypeEnum {
    CpuUsage,
    RamUsage,
}

impl MonitorTypeEnum {
    pub fn icon_label_key(&self) -> &'static str {
        match self {
            MonitorTypeEnum::CpuUsage => "icon-label-cpu-usage",
            MonitorTypeEnum::RamUsage => "icon-label-ram-usage",
        }
    }

    pub fn tray_tooltip_key(&self) -> &'static str {
        match self {
            MonitorTypeEnum::CpuUsage => "tray-tooltip-cpu-usage",
            MonitorTypeEnum::RamUsage => "tray-tooltip-ram-usage",
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            MonitorTypeEnum::CpuUsage | MonitorTypeEnum::RamUsage => "%",
        }
    }
}
