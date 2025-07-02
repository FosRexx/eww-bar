use std::time::Duration;

use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing().with_cpu_usage()),
    );
    let icon: &str = "󰻠";

    loop {
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        // Refresh CPUs again to get actual value.
        sys.refresh_cpu_usage();

        let cpu_usage = sys.global_cpu_usage();

        let style = match cpu_usage {
            100.0 => "color: #8B0000",
            90.0..=100.0 => "color: #FF4500",
            80.0..=90.00 => "color: #FF7A33",
            70.0..=80.00 => "color: #FFA500",
            60.0..=70.00 => "color: #FFC200",
            50.0..=60.00 => "color: #FFD700",
            40.0..=50.00 => "color: #FFEE66",
            30.0..=40.00 => "color: #FFFF99",
            20.0..=30.00 => "color: #FFFFCC",
            10.0..=20.00 => "color: #FFFFFF",
            0.0..=10.0 => "color: #FFFFFF",
            _ => "color: #8B0000",
        };

        println!(
            "(label :style \"{}\" :text \"{} {:5.2}%\")",
            style, icon, cpu_usage
        );
        std::thread::sleep(Duration::from_secs(2));
    }
}
