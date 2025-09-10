use crate::{
    config::app_config::AppConfig,
    enums::monitor_type_enum::MonitorTypeEnum,
    monitor::{cpu_usage_monitor, ram_usage_monitor},
};
use sysinfo::System;

pub trait Monitor {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, sys: &mut System);
    fn get_value(&self) -> f32;
    fn get_type(&self) -> MonitorTypeEnum;
}

pub trait MonitorManager {
    fn update_all(&mut self, app_config: &AppConfig) -> Vec<(MonitorTypeEnum, f32)>;
}

pub struct SystemMonitor {
    sys: System,
    monitors: Vec<Box<dyn Monitor + Send>>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            monitors: vec![
                Box::new(cpu_usage_monitor::CpuUsageMonitor::new()),
                Box::new(ram_usage_monitor::RamUsageMonitor::new()),
            ],
        }
    }
}

impl MonitorManager for SystemMonitor {
    fn update_all(&mut self, app_config: &AppConfig) -> Vec<(MonitorTypeEnum, f32)> {
        self.monitors
            .iter_mut()
            .filter_map(|m| {
                if app_config.active_monitors.contains(&m.get_type()) {
                    m.update(&mut self.sys);
                    Some((m.get_type(), m.get_value()))
                } else {
                    None
                }
            })
            .collect()
    }
}
