use crate::{enums::monitor_type_enum::MonitorTypeEnum, monitor::monitor::Monitor};
use sysinfo::System;

pub struct CpuUsageMonitor {
    value: f32,
}

impl Monitor for CpuUsageMonitor {
    fn new() -> Self {
        Self { value: 0.0 }
    }

    fn update(&mut self, sys: &mut System) {
        sys.refresh_cpu_all();
        self.value = sys.global_cpu_usage();
    }

    fn get_value(&self) -> f32 {
        self.value
    }

    fn get_type(&self) -> MonitorTypeEnum {
        MonitorTypeEnum::CpuUsage
    }
}
