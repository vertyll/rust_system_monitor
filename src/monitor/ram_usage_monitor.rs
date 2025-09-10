use crate::{enums::monitor_type_enum::MonitorTypeEnum, monitor::monitor::Monitor};
use sysinfo::System;

pub struct RamUsageMonitor {
    value: f32,
}

impl Monitor for RamUsageMonitor {
    fn new() -> Self {
        Self { value: 0.0 }
    }

    fn update(&mut self, sys: &mut System) {
        sys.refresh_memory();
        self.value = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
    }

    fn get_value(&self) -> f32 {
        self.value
    }

    fn get_type(&self) -> MonitorTypeEnum {
        MonitorTypeEnum::RamUsage
    }
}
