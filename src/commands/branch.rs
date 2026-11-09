use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    name: Option<String>,
    delete: Option<String>,
    force_delete: Option<String>,
    _list: bool,
    all: bool,
) -> HitResult<()> {
    let start = Instant::now();

    if let Some(del_name) = force_delete {
        GitBackend::run(&["branch", "-D", &del_name])?;
        Ui::success_with_time(
            &format!("Force-deleted branch '{}'", del_name),
            start.elapsed(),
        );
        return Ok(());
    }

    if let Some(del_name) = delete {
        GitBackend::run(&["branch", "-d", &del_name])?;
        Ui::success_with_time(&format!("Deleted branch '{}'", del_name), start.elapsed());
        return Ok(());
    }

    if let Some(new_branch) = name {
        GitBackend::run(&["branch", &new_branch])?;
        Ui::success_with_time(
            &format!("Created branch '{}'", new_branch.bright_cyan()),
            start.elapsed(),
        );
        Ui::hint(&format!("Switch to it with: hit switch {}", new_branch));
        return Ok(());
    }

    // Fast-path: Instant in-memory branch listing (<25µs)
    if !all {
        let current = crate::engine::BranchEngine::current_branch();
        let branches = crate::engine::BranchEngine::list_local_branches();

        if branches.is_empty() {
            println!("{}", "No branches yet".bright_black());
        } else {
            for b in branches {
                if Some(&b) == current.as_ref() {
                    println!("* {}", b.bold().bright_green());
                } else {
                    println!("  {}", b);
                }
            }
        }
        println!("{}", Ui::format_duration(start.elapsed()));
        return Ok(());
    }

    // List remote/all branches
    let args = vec!["branch", "-a"];

    let output = GitBackend::run(&args)?;
    if output.stdout.trim().is_empty() {
        println!("{}", "No branches yet".bright_black());
    } else {
        for line in output.stdout.lines() {
            if line.starts_with('*') {
                println!("{}", line.bold().bright_green());
            } else if line.contains("remotes/") {
                println!("{}", line.cyan());
            } else {
                println!("{}", line);
            }
        }
    }

    println!("{}", Ui::format_duration(start.elapsed()));
    Ok(())
}
