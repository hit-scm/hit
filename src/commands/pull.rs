use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    remote: Option<String>,
    branch: Option<String>,
    rebase: bool,
    force: bool,
) -> HitResult<()> {
    let start = Instant::now();

    let target_remote = remote.clone().unwrap_or_else(|| "origin".to_string());
    let target_branch = branch
        .clone()
        .unwrap_or_else(|| GitBackend::current_branch().unwrap_or_else(|| "HEAD".to_string()));

    // Instant local fast-path: if local branch matches remote-tracking ref, it's already synchronized
    if !force && GitBackend::is_up_to_date_with_remote(&target_branch, &target_remote) {
        let elapsed = start.elapsed();
        Ui::success_with_time(
            &format!(
                "Already up to date with {}/{}",
                target_remote.cyan(),
                target_branch.bold()
            ),
            elapsed,
        );
        return Ok(());
    }

    let mut args = vec!["pull"];

    if rebase {
        args.push("--rebase");
    }

    if let Some(ref r) = remote {
        args.push(r.as_str());
    }

    if let Some(ref b) = branch {
        args.push(b.as_str());
    }

    let title = format!("Pulling from {}/{}", target_remote, target_branch);
    let output = crate::ui::ProgressRunner::run(&title, &args)?;

    let elapsed = start.elapsed();

    let is_up_to_date = output.stdout.contains("Already up to date")
        || output.stderr.contains("Already up to date")
        || output.stdout.contains("Already up-to-date")
        || output.stderr.contains("Already up-to-date")
        || output.stdout.contains("Everything up-to-date")
        || output.stderr.contains("Everything up-to-date");

    if is_up_to_date {
        Ui::success_with_time(
            &format!(
                "Already up to date with {}/{}",
                target_remote.cyan(),
                target_branch.bold()
            ),
            elapsed,
        );
    } else {
        Ui::success_with_time(
            &format!(
                "Pulled latest changes from {}/{}",
                target_remote.cyan(),
                target_branch.bold()
            ),
            elapsed,
        );

        // Display summary of updated files if present
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
