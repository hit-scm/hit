use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    target: Option<String>,
    create_branch: Option<String>,
    detach: bool,
) -> HitResult<()> {
    let start = Instant::now();

    if let Some(new_branch) = create_branch {
        GitBackend::run(&["checkout", "-b", &new_branch])?;
        Ui::success_with_time(
            &format!(
                "Switched to a new branch '{}'",
                new_branch.bold().bright_green()
            ),
            start.elapsed(),
        );
        return Ok(());
    }

    let target_name = match target {
        Some(t) => t,
        None => {
            return Err(HitError::with_hint(
                "Missing branch or target name",
                "Specify the branch or commit to switch to.",
                "Example:\n  hit switch main\nOr create a new branch:\n  hit switch -c <name>",
            ));
        }
    };

    let mut args = vec!["checkout"];
    if detach {
        args.push("--detach");
    }
    args.push(target_name.as_str());

    GitBackend::run(&args)?;

    let elapsed = start.elapsed();
    if detach {
        Ui::success_with_time(
            &format!(
                "Switched to commit {} (detached HEAD)",
                target_name.bold().bright_yellow()
            ),
            elapsed,
        );
        Ui::hint("To keep your work in a branch, run: hit switch -c <branch-name>");
    } else {
        Ui::success_with_time(
            &format!("Switched to branch '{}'", target_name.bold().bright_green()),
            elapsed,
        );
    }

    Ok(())
}
