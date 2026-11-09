use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    remote: Option<String>,
    branch: Option<String>,
    set_upstream: bool,
    force: bool,
    tags: bool,
) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["push"];

    if set_upstream {
        args.push("-u");
    }

    if force {
        args.push("--force");
    }

    if tags {
        args.push("--tags");
    }

    if let Some(ref r) = remote {
        args.push(r.as_str());
    }

    if let Some(ref b) = branch {
        args.push(b.as_str());
    }

    let curr_branch = GitBackend::current_branch().unwrap_or_else(|| "HEAD".to_string());
    let target_remote = remote.clone().unwrap_or_else(|| "origin".to_string());
    let target_branch = branch.clone().unwrap_or_else(|| curr_branch.clone());

    // Instant local fast-path: if local and remote-tracking commit hashes match, zero unpushed commits exist
    if !force
        && !tags
        && !set_upstream
        && GitBackend::is_up_to_date_with_remote(&target_branch, &target_remote)
    {
        let elapsed = start.elapsed();
        Ui::success_with_time(
            &format!(
                "Everything up to date (branch '{}' is synchronized with {}/{})",
                target_branch.bold(),
                target_remote.cyan(),
                target_branch.bold()
            ),
            elapsed,
        );
        return Ok(());
    }

    let title = if tags {
        format!("Pushing tags to {}", target_remote)
    } else {
        format!("Pushing {} -> {}", target_branch, target_remote)
    };

    let output = crate::ui::ProgressRunner::run(&title, &args)?;

    let elapsed = start.elapsed();

    // Check if git reported Everything up-to-date
    let is_up_to_date = output.stderr.contains("Everything up-to-date")
        || output.stdout.contains("Everything up-to-date");

    if is_up_to_date {
        Ui::success_with_time(
            &format!(
                "Everything up to date with {}/{}",
                target_remote.cyan(),
                target_branch.bold()
            ),
            elapsed,
        );
        return Ok(());
    }

    let summary = if tags {
        format!("Pushed tags to {}", target_remote.cyan())
    } else {
        format!(
            "Pushed {} -> {}/{}",
            target_branch.bold(),
            target_remote.cyan(),
            target_branch.bold()
        )
    };

    Ui::success_with_time(&summary, elapsed);

    // If git produced informative status lines (new branch, updates), print them cleanly
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
