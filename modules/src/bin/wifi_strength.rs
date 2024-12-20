use std::process::Command;

fn main() {
    let iw_output = Command::new("iw")
        .args(["wlan0", "link"])
        .output()
        .expect("Failed to execute 'iw' command");

    if !iw_output.status.success() {
        eprintln!("'iw' command failed to run successfully.");
        return;
    }

    let iw_result = match String::from_utf8(iw_output.stdout) {
        Ok(output) => output,
        Err(_) => {
            eprintln!("Failed to parse 'iw' command output as UTF-8.");
            return;
        }
    };

    let mut icon: String = "󰤭".to_string();

    if iw_result.contains("Not connected") {
        println!("(label :style \"color: red\" :text \"{} down\")", icon);
        return;
    }

    let signal_dbm = match extract_value(&iw_result, "signal") {
        Some(signal) => match signal.parse::<i8>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to parse signal strength.");
                return;
            }
        },
        None => {
            eprintln!("Signal strength not found in 'iw' output.");
            return;
        }
    };

    let freq_mhz = match extract_value(&iw_result, "freq") {
        Some(freq) => match freq.parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to parse frequency.");
                return;
            }
        },
        None => {
            eprintln!("Frequency not found in 'iw' output.");
            return;
        }
    };

    let quality = (2 * (signal_dbm + 100)) as u8;

    icon = match freq_mhz {
        5150.0..=5925.0 => match_quality(quality, "󰩯"),
        2400.0..=2484.0 => match_quality(quality, "󰜔"),
        _ => {
            eprintln!("Frequency band not supported.");
            return;
        }
    };

    let style = match quality {
        80.. => "color: #00FF00",
        60..=79 => "color: #7FFF00",
        40..=59 => "color: #FFFF00",
        20..=39 => "color: #FF7F00",
        0..=19 => "color: #FF0000",
    };

    println!(
        "(label :style \"{}\" :text \"{} {}%\")",
        style, icon, quality
    );
}

fn extract_value<'a>(output: &'a str, key: &str) -> Option<&'a str> {
    output
        .lines()
        .find(|line| line.contains(key))
        .and_then(|line| line.split_whitespace().nth(1))
}

fn match_quality(quality: u8, band_icon: &str) -> String {
    match quality {
        80.. => format!("󰤨 {}", band_icon),
        60..=79 => format!("󰤥 {}", band_icon),
        40..=59 => format!("󰤢 {}", band_icon),
        20..=39 => format!("󰤟 {}", band_icon),
        0..=19 => format!("󰤯 {}", band_icon),
    }
}
