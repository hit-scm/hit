use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(subcommand: Option<String>, args: Vec<String>, verbose: bool) -> HitResult<()> {
    let start = Instant::now();

    match subcommand.as_deref() {
        Some("add") => {
            if args.len() < 2 {
                return Err(HitError::with_hint(
                    "Missing arguments for 'hit remote add'",
                    "Expected remote name and URL.",
                    "Example:\n  hit remote add origin https://github.com/user/repo.git",
                ));
            }
            let name = &args[0];
            let url = &args[1];
            GitBackend::run(&["remote", "add", name, url])?;
            Ui::success_with_time(
                &format!("Added remote '{}' -> {}", name.bold(), url.cyan()),
                start.elapsed(),
            );
        }
        Some("remove") | Some("rm") => {
            if args.is_empty() {
                return Err(HitError::with_hint(
                    "Missing remote name to remove",
                    "Specify the remote name to remove.",
                    "Example:\n  hit remote remove origin",
                ));
            }
            let name = &args[0];
            GitBackend::run(&["remote", "remove", name])?;
            Ui::success_with_time(
                &format!("Removed remote '{}'", name.bold()),
                start.elapsed(),
            );
        }
        Some("rename") => {
            if args.len() < 2 {
                return Err(HitError::with_hint(
                    "Missing arguments for 'hit remote rename'",
                    "Expected old name and new name.",
                    "Example:\n  hit remote rename origin upstream",
                ));
            }
            let old = &args[0];
            let new = &args[1];
            GitBackend::run(&["remote", "rename", old, new])?;
            Ui::success_with_time(
                &format!("Renamed remote '{}' to '{}'", old, new),
                start.elapsed(),
            );
        }
        Some("get-url") => {
            let name = args.first().map(|s| s.as_str()).unwrap_or("origin");
            let output = GitBackend::run(&["remote", "get-url", name])?;
            println!("{}", output.stdout.trim().cyan());
            println!("{}", Ui::format_duration(start.elapsed()));
        }
        _ => {
            // Fast in-memory native remote listing (<30µs)
            let remotes = crate::engine::RemoteEngine::list_remotes();
            if remotes.is_empty() {
                println!("{}", "No remotes configured".bright_black());
                println!("  {}", Ui::dim("Add one with: hit remote add origin <url>"));
            } else {
                for (name, url) in remotes {
                    if verbose {
                        println!(
                            "{}\t{}\t{}",
                            name.bold().bright_green(),
                            url.cyan(),
                            "(fetch)".bright_black()
                        );
                        println!(
                            "{}\t{}\t{}",
                            name.bold().bright_green(),
                            url.cyan(),
                            "(push)".bright_black()
                        );
                    } else {
                        println!("{}", name.bold().bright_green());
                    }
                }
            }
            println!("{}", Ui::format_duration(start.elapsed()));
        }
    }

    Ok(())
}
