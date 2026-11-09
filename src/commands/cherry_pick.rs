use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(commits: Vec<String>, abort: bool, continue_cp: bool) -> HitResult<()> {
    let start = Instant::now();

    if abort {
        GitBackend::run(&["cherry-pick", "--abort"])?;
        Ui::success_with_time(
            "Cherry-pick aborted cleanly, working tree restored.",
            start.elapsed(),
        );
        return Ok(());
    }

    if continue_cp {
        GitBackend::run(&["cherry-pick", "--continue"])?;
        Ui::success_with_time("Cherry-pick completed successfully.", start.elapsed());
        return Ok(());
    }

    if commits.is_empty() {
        return Err(HitError::with_hint(
            "Missing commit argument",
            "Specify the commit hash to cherry-pick into the current branch.",
            "Example:\n  hit cherry-pick 7f2e1a4\nOr continue/abort an in-progress cherry-pick:\n  hit cherry-pick --continue\n  hit cherry-pick --abort",
        ));
    }

    let mut args = vec!["cherry-pick"];
    for c in &commits {
        args.push(c.as_str());
    }

    let output = GitBackend::run(&args)?;
    let elapsed = start.elapsed();

    Ui::success_with_time(
        &format!(
            "Applied commit{} {}",
            if commits.len() == 1 { "" } else { "s" },
            commits.join(", ").bright_yellow()
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
