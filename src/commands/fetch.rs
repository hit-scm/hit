use crate::error::HitResult;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(remote: Option<String>, all: bool, prune: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["fetch"];

    if all {
        args.push("--all");
    }

    if prune {
        args.push("--prune");
    }

    if let Some(ref r) = remote {
        args.push(r.as_str());
    }

    let target = if all {
        "all remotes".to_string()
    } else {
        remote.clone().unwrap_or_else(|| "origin".to_string())
    };

    let title = format!("Fetching from {}", target);
    let output = crate::ui::ProgressRunner::run(&title, &args)?;

    let elapsed = start.elapsed();

    Ui::success_with_time(
        &format!("Fetched latest refs from {}", target.cyan()),
        elapsed,
    );

    for line in output.stderr.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("* [new branch]")
            || trimmed.starts_with("* [new tag]")
            || trimmed.contains("->")
        {
            println!("  {}", trimmed.bright_black());
        }
    }

    Ok(())
}
