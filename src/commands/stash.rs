use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(action: Option<String>, message: Option<String>) -> HitResult<()> {
    let start = Instant::now();
    let action_str = action.unwrap_or_else(|| "push".to_string());

    match action_str.as_str() {
        "push" | "save" => {
            let mut args = vec!["stash", "push"];
            if let Some(ref m) = message {
                args.push("-m");
                args.push(m.as_str());
            }
            let output = GitBackend::run(&args)?;
            let elapsed = start.elapsed();
            if output.stdout.contains("No local changes to save") {
                println!("No local changes to save {}", Ui::format_duration(elapsed));
            } else {
                Ui::success_with_time("Saved working directory and index state", elapsed);
            }
        }
        "pop" => {
            let output = GitBackend::run(&["stash", "pop"])?;
            Ui::success_with_time("Applied and dropped stash@{0}", start.elapsed());
            for line in output.stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Auto-merging") || trimmed.contains("modified:") {
                    println!("  {}", trimmed.bright_black());
                }
            }
        }
        "apply" => {
            let output = GitBackend::run(&["stash", "apply"])?;
            Ui::success_with_time(
                "Applied stash@{0} (stash retained in list)",
                start.elapsed(),
            );
            for line in output.stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Auto-merging") || trimmed.contains("modified:") {
                    println!("  {}", trimmed.bright_black());
                }
            }
        }
        "show" => {
            let output = GitBackend::run(&["stash", "show", "-p"])?;
            for line in output.stdout.lines() {
                if line.starts_with('+') && !line.starts_with("+++") {
                    println!("{}", line.bright_green());
                } else if line.starts_with('-') && !line.starts_with("---") {
                    println!("{}", line.bright_red());
                } else {
                    println!("{}", line);
                }
            }
            println!("{}", Ui::format_duration(start.elapsed()));
        }
        "list" => {
            let output = GitBackend::run(&["stash", "list"])?;
            if output.stdout.trim().is_empty() {
                println!("{}", "No stashes found".bright_black());
            } else {
                for line in output.stdout.lines() {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        println!("{}: {}", parts[0].bright_yellow(), parts[1]);
                    } else {
                        println!("{}", line);
                    }
                }
            }
            println!("{}", Ui::format_duration(start.elapsed()));
        }
        "clear" => {
            GitBackend::run(&["stash", "clear"])?;
            Ui::success_with_time("Cleared all stashes", start.elapsed());
        }
        "drop" => {
            GitBackend::run(&["stash", "drop"])?;
            Ui::success_with_time("Dropped stash@{0}", start.elapsed());
        }
        _ => {
            let mut args = vec!["stash", action_str.as_str()];
            if let Some(ref m) = message {
                args.push(m.as_str());
            }
            GitBackend::run(&args)?;
            println!("{}", Ui::format_duration(start.elapsed()));
        }
    }

    Ok(())
}
