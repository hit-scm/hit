use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::path::Path;
use std::time::Instant;

pub fn execute(staged: bool, hard: bool) -> HitResult<()> {
    let start = Instant::now();

    if !GitBackend::is_in_repo() {
        return Err(HitError::not_in_repo());
    }

    if hard {
        GitBackend::run(&["reset", "--hard", "HEAD"])?;
        Ui::success_with_time(
            "Discarded all uncommitted changes and restored working tree to HEAD",
            start.elapsed(),
        );
        return Ok(());
    }

    if staged {
        GitBackend::run(&["reset", "HEAD"])?;
        Ui::success_with_time(
            "Unstaged all changes (working directory preserved)",
            start.elapsed(),
        );
        return Ok(());
    }

    // 1. Check if merge is in progress
    if Path::new(".git/MERGE_HEAD").exists() {
        GitBackend::run(&["merge", "--abort"])?;
        Ui::success_with_time(
            "Aborted in-progress merge and restored working directory",
            start.elapsed(),
        );
        return Ok(());
    }

    // 2. Check if rebase is in progress
    if Path::new(".git/rebase-merge").exists() || Path::new(".git/rebase-apply").exists() {
        GitBackend::run(&["rebase", "--abort"])?;
        Ui::success_with_time(
            "Aborted in-progress rebase and restored working directory",
            start.elapsed(),
        );
        return Ok(());
    }

    // 3. Check status for staged changes vs undoing commit
    let status_out = GitBackend::run(&["status", "--porcelain=v2"])?;
    let has_staged = status_out.stdout.lines().any(|l| {
        if l.starts_with("1 ") || l.starts_with("2 ") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 2 {
                let x = parts[1].chars().next().unwrap_or('.');
                return x == 'M' || x == 'A' || x == 'D' || x == 'R';
            }
        }
        false
    });

    let head_log = GitBackend::run(&["log", "-1", "--oneline"]);

    match head_log {
        Ok(log) if !log.stdout.trim().is_empty() => {
            let commit_line = log.stdout.trim().to_string();
            // Undo last commit safely with soft reset (preserves all changes staged)
            GitBackend::run(&["reset", "--soft", "HEAD~1"])?;
            let elapsed = start.elapsed();
            Ui::success_with_time(
                &format!(
                    "Undid commit {} (changes preserved in staging index)",
                    commit_line.bold()
                ),
                elapsed,
            );
            Ui::hint(
                "Your changes are still staged. Make adjustments and run 'hit commit -m \"...\"'",
            );
        }
        _ => {
            if has_staged {
                // Unstage everything
                GitBackend::run(&["reset"])?;
                Ui::success_with_time(
                    "Undid staging (all changes un-staged, working directory intact)",
                    start.elapsed(),
                );
            } else {
                return Err(HitError::with_hint(
                    "Nothing to undo",
                    "There are no commits or in-progress operations to undo in this repository.",
                    "Make changes or a commit first before running 'hit undo'.",
                ));
            }
        }
    }

    Ok(())
}
