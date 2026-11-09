use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(dry_run: bool, force: bool) -> HitResult<()> {
    let start = Instant::now();

    if !dry_run && !force {
        return Err(HitError::with_hint(
            "Clean requires explicit confirmation",
            "To prevent accidental data loss, 'hit clean' requires either -n (dry-run) or -f (force).",
            "Preview files to be removed:\n  hit clean -n\nRemove untracked files permanently:\n  hit clean -f",
        ));
    }

    let mut args = vec!["clean", "-d"];
    if dry_run {
        args.push("-n");
    }
    if force {
        args.push("-f");
    }

    let output = GitBackend::run(&args)?;
    let elapsed = start.elapsed();

    if output.stdout.trim().is_empty() {
        println!(
            "No untracked files to clean {}",
            Ui::format_duration(elapsed)
        );
    } else {
        if dry_run {
            println!(
                "{}",
                "Would remove the following untracked files:"
                    .bold()
                    .yellow()
            );
        } else {
            Ui::success_with_time("Cleaned untracked files:", elapsed);
        }

        for line in output.stdout.lines() {
            println!("  {}", line.bright_red());
        }

        if dry_run {
            Ui::hint("Run 'hit clean -f' to permanently delete these files.");
        }
    }

    Ok(())
}
