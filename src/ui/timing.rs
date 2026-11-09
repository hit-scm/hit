use colored::*;
use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    let micros = d.as_micros();
    let formatted = if micros < 1000 {
        format!("{}µs", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    };

    format!("[{}]", formatted).bright_black().to_string()
}
