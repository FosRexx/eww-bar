extern crate battery;

use battery::units::ratio::percent;
use battery::units::time::second;
use std::io;

// Helper function to format time (hh:mm)
fn format_time(seconds: Option<f32>) -> String {
    seconds
        .map(|s| {
            let total_seconds = s as u32;
            let minutes = (total_seconds / 60) % 60;
            let hours = total_seconds / 3600;
            format!("({:02}:{:02})", hours, minutes)
        })
        .unwrap_or_else(|| "N/A".to_string())
}

// Get appropriate icon based on state and charge
fn get_icon(state: &str, charge: u32) -> (&str, &str) {
    match state {
        "charging" => (" 󰂄", "color: #00FFFF"),
        "full" | "unknown" => ("󰂄", "color: #00FFFF"),
        "discharging" => match charge {
            100 => ("󰁹", "color: #FFFFFF"),
            90..=99 => ("󰂂", "color: #FFFFFF"),
            80..=89 => ("󰂁", "color: #FFFFCC"),
            70..=79 => ("󰂀", "color: #FFFF99"),
            60..=69 => ("󰁿", "color: #FFEE66"),
            50..=59 => ("󰁾", "color: #FFD700"),
            40..=49 => ("󰁽", "color: #FFC200"),
            30..=39 => ("󰁼", "color: #FFA500"),
            20..=29 => ("󰁻", "color: #FF7A33"),
            10..=19 => ("󰁺", "color: #FF4500"),
            0..=9 => ("󰂃", "color: #8B0000"),
            _ => ("󰂃", "color: #8B0000"),
        },
        _ => ("󰂃", "color: #8B0000"),
    }
}

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Error accessing battery information: {}", e);
            return Err(e);
        }
        None => {
            eprintln!("No batteries found.");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    let state: String = battery.state().to_string();
    let state_of_charge: u32 = battery.state_of_charge().get::<percent>() as u32;
    let time_to_full: String = format_time(battery.time_to_full().map(|t| t.get::<second>()));
    let time_to_empty: String = format_time(battery.time_to_empty().map(|t| t.get::<second>()));
    let (icon, style): (&str, &str) = get_icon(&state, state_of_charge);

    match state.as_str() {
        "charging" => {
            print!("(box :spacing 5 :space-evenly false :orientation \"h\" (label :text \"{}\") (label :style \"{}\" :text \"{}%\") (label :text \"{}\"))", icon, style, state_of_charge, time_to_full);
        }
        "discharging" => {
            print!("(box :spacing 5 :space-evenly false :orientation \"h\" (label :text \"{}\") (label :style \"{}\" :text \"{}%\") (label :text \"{}\"))", icon, style, state_of_charge, time_to_empty);
        }
        "unknown" | "full" => {
            print!(
                "(box :spacing 5 :space-evenly false :orientation \"h\" (label :text \"{}\") (label :style \"{}\" :text \"{}%\"))",
                icon, style, state_of_charge
            );
        }
        _ => {
            eprintln!("Unexpected battery state: {}", state);
        }
    }

    Ok(())
}
