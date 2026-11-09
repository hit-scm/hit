use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(message: Option<String>, all: bool, amend: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["commit"];

    if all {
        args.push("-a");
    }

    if amend {
        args.push("--amend");
    }

    let msg = match message {
        Some(m) if !m.trim().is_empty() => m,
        _ => {
            if !amend {
                return Err(HitError::with_hint(
                    "Missing commit message",
                    "Hit requires a non-empty commit message to record changes.",
                    "Provide a message with 'hit commit -m \"Your message\"'.",
                ));
            } else {
                String::new()
            }
        }
    };

    if !msg.is_empty() {
        args.push("-m");
        args.push(msg.as_str());
    }

    let output = GitBackend::run(&args)?;

    let elapsed = start.elapsed();

    // Parse git output for clean display:
    // [main (root-commit) 46e47f1] feat: initial commit
    // 2 files changed, 2 insertions(+)
    let lines: Vec<&str> = output.stdout.lines().collect();
    if let Some(first_line) = lines.first() {
        println!("{} {}", first_line.bold(), Ui::format_duration(elapsed));
    }

    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if trimmed.contains("changed")
            || trimmed.contains("insertion")
            || trimmed.contains("deletion")
        {
            println!("{}", crate::ui::diff_stats::colorize_diff_summary(trimmed));
        }
    }

    Ok(())
}
