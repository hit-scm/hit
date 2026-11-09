use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    upstream: Option<String>,
    continue_rebase: bool,
    abort_rebase: bool,
) -> HitResult<()> {
    let start = Instant::now();

    if abort_rebase {
        GitBackend::run(&["rebase", "--abort"])?;
        Ui::success_with_time("Rebase aborted cleanly, branch restored.", start.elapsed());
        return Ok(());
    }

    if continue_rebase {
        let output = GitBackend::run(&["rebase", "--continue"])?;
        Ui::success_with_time("Rebase continued successfully.", start.elapsed());
        for line in output.stdout.lines() {
            println!("  {}", line.bright_black());
        }
        return Ok(());
    }

    let target = match upstream {
        Some(u) => u,
        None => {
            return Err(HitError::with_hint(
                "Missing upstream branch for rebase",
                "Specify the target branch to rebase current branch onto.",
                "Example:\n  hit rebase main\nOr continue/abort an in-progress rebase:\n  hit rebase --continue\n  hit rebase --abort",
            ));
        }
    };

    let output = GitBackend::run(&["rebase", target.as_str()])?;
    let elapsed = start.elapsed();
    let current = GitBackend::current_branch().unwrap_or_else(|| "HEAD".to_string());

    Ui::success_with_time(
        &format!(
            "Successfully rebased '{}' onto '{}'",
            current.bold().bright_green(),
            target.cyan()
        ),
        elapsed,
    );

    for line in output.stdout.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            println!("  {}", trimmed.bright_black());
        }
    }

    Ok(())
}
