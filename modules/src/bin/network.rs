use std::{
    thread,
    time::{Duration, Instant},
};
use sysinfo::Networks;

// Format bytes into human-readable string
fn format_rate(bytes_per_sec: f64) -> String {
    match bytes_per_sec {
        b if b < 1024.0 => format!("{:.1} B/s", b),
        b if b < 1_048_576.0 => format!("{:.1}KB/s", b / 1024.0),
        b if b < 1_073_741_824.0 => format!("{:.1}MB/s", b / 1_048_576.0),
        b if b < 1_099_511_627_776.0 => format!("{:.1}GB/s", b / 1_073_741_824.0),
        _ => "inf B/s".to_string(),
    }
}

fn main() {
    const POLL_INTERVAL: Duration = Duration::from_secs(2);

    let mut networks = Networks::new_with_refreshed_list();

    // Record the starting time
    let mut last_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now - last_time;
        last_time = now;

        // Sleep until the next poll
        thread::sleep(POLL_INTERVAL);
        // Refresh network data
        networks.refresh(true);

        let mut total_received: f64 = 0.0;
        let mut total_transmitted: f64 = 0.0;

        // Calculate total bytes received and transmitted
        for (_, network) in &networks {
            total_received += network.received() as f64;
            total_transmitted += network.transmitted() as f64;
        }

        // Compute rates per second
        total_received /= elapsed.as_secs_f64();
        total_transmitted /= elapsed.as_secs_f64();

        // Format for display
        let received_display = format_rate(total_received);
        let transmitted_display = format_rate(total_transmitted);

        println!(" {:>9}  {:>9}", received_display, transmitted_display);
    }
}
