use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(remote: Option<String>) -> HitResult<()> {
    let start = Instant::now();

    if !GitBackend::is_in_repo() {
        return Err(HitError::not_in_repo());
    }

    let branch = match GitBackend::current_branch() {
        Some(b) => b,
        None => {
            return Err(HitError::with_hint(
                "Cannot sync in detached HEAD state",
                "You are currently not on any branch.",
                "Switch to a branch first with: hit switch <branch-name>",
            ));
        }
    };

    let target_remote = remote.unwrap_or_else(|| "origin".to_string());

    // 1. Check for dirty working tree (tracked modifications or staged changes)
    let status = GitBackend::run(&["status", "--porcelain=v2"])?;
    let is_dirty = status
        .stdout
        .lines()
        .any(|l| l.starts_with("1 ") || l.starts_with("2 ") || l.starts_with("u "));

    // Instant local fast-path: if clean and already synchronized, skip remote pull/push entirely
    if !is_dirty && GitBackend::is_up_to_date_with_remote(&branch, &target_remote) {
        let elapsed = start.elapsed();
        Ui::success_with_time(
            &format!(
                "Repository is already synchronized with {}/{} (no changes)",
                target_remote.cyan(),
                branch.bold()
            ),
            elapsed,
        );
        return Ok(());
    }

    if is_dirty {
        println!(
            "  {} Saving local working state temporarily...",
            "•".bright_blue()
        );
        GitBackend::run(&["stash", "push", "-m", "hit-sync-auto-stash"])?;
    }

    // 2. Pull remote changes with rebase
    let pull_title = format!("Syncing (pull): fetching {}/{}", target_remote, branch);
    let pull_res =
        crate::ui::ProgressRunner::run(&pull_title, &["pull", "--rebase", &target_remote, &branch]);

    let pull_output = match pull_res {
        Ok(out) => out,
        Err(e) => {
            if is_dirty {
                let _ = GitBackend::run(&["stash", "pop"]);
            }
            return Err(e);
        }
    };

    let pull_was_up_to_date = pull_output.stdout.contains("Already up to date")
        || pull_output.stderr.contains("Already up to date");

    // 3. Push local commits only if not already synchronized
    let is_already_synced = GitBackend::is_up_to_date_with_remote(&branch, &target_remote);
    let mut pushed = false;

    if !is_already_synced {
        let push_title = format!("Syncing (push): publishing to {}/{}", target_remote, branch);
        let push_res =
            crate::ui::ProgressRunner::run(&push_title, &["push", &target_remote, &branch]);

        if let Err(e) = push_res {
            if is_dirty {
                let _ = GitBackend::run(&["stash", "pop"]);
            }
            return Err(e);
        }
        pushed = true;
    }

    // 4. Restore stashed changes if any
    if is_dirty {
        println!("  {} Restoring local working state...", "•".bright_blue());
        let _ = GitBackend::run(&["stash", "pop"]);
    }

    let elapsed = start.elapsed();
    if pull_was_up_to_date && !pushed {
        Ui::success_with_time(
            &format!(
                "Repository is already synchronized with {}/{} (no changes)",
                target_remote.cyan(),
                branch.bold()
            ),
            elapsed,
        );
    } else {
        Ui::success_with_time(
            &format!(
                "Synced branch '{}' with {}/{}",
                branch.bold().bright_green(),
                target_remote.cyan(),
                branch.bold()
            ),
            elapsed,
        );
    }

    Ok(())
}
