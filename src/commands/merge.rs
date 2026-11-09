use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(branch: Option<String>, abort: bool, no_ff: bool) -> HitResult<()> {
    let start = Instant::now();

    if abort {
        GitBackend::run(&["merge", "--abort"])?;
        Ui::success_with_time(
            "Merge aborted cleanly, working tree restored.",
            start.elapsed(),
        );
        return Ok(());
    }

    let target_branch = match branch {
        Some(b) => b,
        None => {
            return Err(HitError::with_hint(
                "Missing branch to merge",
                "Specify the branch you want to merge into current branch.",
                "Example:\n  hit merge feature/auth",
            ));
        }
    };

    let mut args = vec!["merge"];
    if no_ff {
        args.push("--no-ff");
    }
    args.push(target_branch.as_str());

    let output = GitBackend::run(&args)?;

    let elapsed = start.elapsed();
    let current = GitBackend::current_branch().unwrap_or_else(|| "HEAD".to_string());

    if output.stdout.contains("Already up to date") {
        println!("Already up to date {}", Ui::format_duration(elapsed));
    } else {
        Ui::success_with_time(
            &format!(
                "Merged '{}' into '{}'",
                target_branch.bold().cyan(),
                current.bold().bright_green()
            ),
            elapsed,
        );
        for line in output.stdout.lines() {
            let trimmed = line.trim();
            if trimmed.contains("changed")
                || trimmed.contains("insertion")
                || trimmed.contains("deletion")
            {
                println!("{}", crate::ui::diff_stats::colorize_diff_summary(trimmed));
            }
        }
    }

    Ok(())
}
