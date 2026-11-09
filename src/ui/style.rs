use colored::*;
use std::time::Duration;

pub struct Ui;

impl Ui {
    pub fn format_duration(d: Duration) -> String {
        crate::ui::timing::format_duration(d)
    }

    pub fn print_timing(d: Duration) {
        println!("{}", Self::format_duration(d));
    }

    pub fn success(msg: &str) {
        println!("{} {}", "✔".bright_green().bold(), msg.bold());
    }

    pub fn success_with_time(msg: &str, elapsed: Duration) {
        println!(
            "{} {} {}",
            "✔".bright_green().bold(),
            msg.bold(),
            Self::format_duration(elapsed)
        );
    }

    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".bright_blue().bold(), msg);
    }

    pub fn warn(msg: &str) {
        println!("{} {}", "⚠".bright_yellow().bold(), msg);
    }

    pub fn hint(msg: &str) {
        println!("  {} {}", "hint:".bold().bright_yellow(), msg.cyan());
    }

    pub fn dim(msg: &str) -> String {
        msg.bright_black().to_string()
    }
}
