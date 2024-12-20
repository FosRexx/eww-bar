use sysinfo::{MemoryRefreshKind, RefreshKind, System};

fn main() {
    let sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
    );

    let icon: &str = "";

    let used_memory = sys.used_memory() as f64 / 1_073_741_824.0;
    let total_memory = sys.total_memory() as f64 / 1_073_741_824.0;

    let style = match used_memory {
        m if m > 13.7 => "color: #8B0000",
        m if m > 12.2 => "color: #FF4500",
        m if m > 10.7 => "color: #FF7A33",
        _ => "",
    };

    print!(
        "(label :style \"{}\" :text \"{} {:4.1}G/{:.1}G\")",
        style, icon, used_memory, total_memory
    );
}
