use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::{ProgressRunner, Ui};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Default, Clone)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
}

/// Parse `.gitmodules` file if present in the repository root
pub fn parse_gitmodules() -> HashMap<String, SubmoduleInfo> {
    let mut map = HashMap::new();
    let gitmodules_path = Path::new(".gitmodules");
    if !gitmodules_path.exists() {
        return map;
    }

    let content = match fs::read_to_string(gitmodules_path) {
        Ok(c) => c,
        Err(_) => return map,
    };

    let mut current_name = String::new();
    let mut current_path = String::new();
    let mut current_url = String::new();
    let mut current_branch = None;

    let flush = |map: &mut HashMap<String, SubmoduleInfo>,
                 name: &str,
                 path: &str,
                 url: &str,
                 branch: Option<&str>| {
        if !path.is_empty() {
            map.insert(
                path.to_string(),
                SubmoduleInfo {
                    name: name.to_string(),
                    path: path.to_string(),
                    url: url.to_string(),
                    branch: branch.map(|s| s.to_string()),
                },
            );
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("[submodule \"") && trimmed.ends_with("\"]") {
            // Flush previous entry
            flush(
                &mut map,
                &current_name,
                &current_path,
                &current_url,
                current_branch.as_deref(),
            );
            current_name = trimmed[12..trimmed.len() - 2].to_string();
            current_path.clear();
            current_url.clear();
            current_branch = None;
        } else if let Some(idx) = trimmed.find('=') {
            let key = trimmed[..idx].trim();
            let value = trimmed[idx + 1..].trim();
            match key {
                "path" => current_path = value.to_string(),
                "url" => current_url = value.to_string(),
                "branch" => current_branch = Some(value.to_string()),
                _ => {}
            }
        }
    }

    flush(
        &mut map,
        &current_name,
        &current_path,
        &current_url,
        current_branch.as_deref(),
    );

    map
}

pub fn execute(
    action: Option<String>,
    args: Vec<String>,
    init: bool,
    recursive: bool,
    force: bool,
    branch: Option<String>,
) -> HitResult<()> {
    if !GitBackend::is_in_repo() {
        return Err(HitError::not_in_repo());
    }

    let subcmd = action.as_deref().unwrap_or("status");

    match subcmd {
        "status" | "list" => status(recursive),
        "add" => add(args, branch, force),
        "update" | "up" => update(args, init, recursive, force),
        "init" => initialize(args),
        "sync" => sync(args, recursive),
        "deinit" => deinit(args, force),
        "foreach" => foreach(args),
        unknown => {
            let valid: Vec<String> = [
                "status", "add", "update", "init", "sync", "deinit", "foreach",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            let closest = crate::error::hints::find_closest(unknown, &valid);
            let hint = match closest {
                Some(sim) => format!(
                    "Did you mean 'hit submodule {}'?\nAvailable actions: status, add, update, init, sync, deinit, foreach",
                    sim
                ),
                None => "Available actions: status, add, update, init, sync, deinit, foreach".to_string(),
            };
            Err(HitError::with_hint(
                format!("Unknown submodule action '{}'", unknown),
                format!("'hit submodule {}' is not a recognized command.", unknown),
                hint,
            ))
        }
    }
}

pub fn status(_recursive: bool) -> HitResult<()> {
    let start = Instant::now();
    let configs = parse_gitmodules();

    let output = match GitBackend::run(&["submodule", "status"]) {
        Ok(out) => out,
        Err(_) => {
            println!("No submodules found in repository.");
            Ui::hint("Add a submodule with:\n  hit submodule add <url> [path]");
            return Ok(());
        }
    };

    let lines: Vec<&str> = output
        .stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    if lines.is_empty() && configs.is_empty() {
        println!("No submodules found in repository.");
        Ui::hint("Add a submodule with:\n  hit submodule add <url> [path]");
        return Ok(());
    }

    println!(
        "{} ({} tracked):",
        "Submodules".bold(),
        lines.len().to_string().cyan()
    );

    let mut has_uninitialized = false;
    let mut has_modified = false;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let flag = line.chars().next().unwrap_or(' ');
        let rest = &line[1..];
        let mut parts = rest.split_whitespace();
        let commit_full = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        let describe = parts.collect::<Vec<&str>>().join(" ");

        let short_sha = if commit_full.len() >= 7 {
            &commit_full[..7]
        } else {
            commit_full
        };

        let info = configs.get(path);
        let url_display = info
            .map(|i| format!(" -> {}", i.url.dimmed()))
            .unwrap_or_default();
        let branch_display = info
            .and_then(|i| i.branch.as_ref())
            .map(|b| format!(" ({})", b.cyan()))
            .or_else(|| {
                if !describe.is_empty() {
                    Some(format!(" {}", describe.dimmed()))
                } else {
                    None
                }
            })
            .unwrap_or_default();

        match flag {
            '-' => {
                has_uninitialized = true;
                println!(
                    "  {} {} {}{}{} {}",
                    "○".bright_yellow(),
                    path.bold(),
                    short_sha.yellow(),
                    branch_display,
                    url_display,
                    "[uninitialized]".bright_yellow()
                );
            }
            '+' => {
                has_modified = true;
                println!(
                    "  {} {} {}{}{} {}",
                    "●".bright_blue(),
                    path.bold(),
                    short_sha.blue(),
                    branch_display,
                    url_display,
                    "[modified/checked out at different commit]".bright_blue()
                );
            }
            'U' => {
                println!(
                    "  {} {} {}{}{} {}",
                    "✖".bright_red(),
                    path.bold(),
                    short_sha.red(),
                    branch_display,
                    url_display,
                    "[merge conflict]".bright_red()
                );
            }
            _ => {
                println!(
                    "  {} {} {}{}{} {}",
                    "✔".bright_green(),
                    path.bold(),
                    short_sha.green(),
                    branch_display,
                    url_display,
                    "[clean]".dimmed()
                );
            }
        }
    }

    println!();
    Ui::success_with_time("Submodule status checked.", start.elapsed());

    if has_uninitialized {
        Ui::hint("To initialize and checkout submodules, run:\n  hit submodule update --init --recursive");
    } else if has_modified {
        Ui::hint("To stage updated submodule references, run:\n  hit add <submodule-path> && hit commit -m \"update submodule\"");
    }

    Ok(())
}

pub fn add(args: Vec<String>, branch: Option<String>, force: bool) -> HitResult<()> {
    let start = Instant::now();

    if args.is_empty() {
        return Err(HitError::with_hint(
            "Missing repository URL",
            "Provide the remote URL of the submodule you wish to add.",
            "Example:\n  hit submodule add https://github.com/org/library.git [path]\n  hit submodule add -b main https://github.com/org/library.git vendor/lib",
        ));
    }

    let url = &args[0];
    let custom_path = args.get(1);

    let path_display = match custom_path {
        Some(p) => p.clone(),
        None => {
            let trimmed = url.trim_end_matches('/').trim_end_matches(".git");
            trimmed
                .rsplit('/')
                .next()
                .unwrap_or("submodule")
                .to_string()
        }
    };

    let mut git_args = vec!["submodule", "add"];
    if force {
        git_args.push("-f");
    }

    let branch_str;
    if let Some(ref b) = branch {
        git_args.push("-b");
        branch_str = b.clone();
        git_args.push(&branch_str);
    }

    git_args.push(url.as_str());
    if let Some(p) = custom_path {
        git_args.push(p.as_str());
    }

    let title = format!("Adding submodule {}", path_display);
    ProgressRunner::run(&title, &git_args)?;

    let elapsed = start.elapsed();
    Ui::success_with_time(
        &format!(
            "Added submodule '{}' from {}",
            path_display.bold(),
            url.cyan()
        ),
        elapsed,
    );

    Ui::hint(&format!(
        "Submodule configured in .gitmodules. Commit the addition with:\n  hit commit -m \"add submodule {}\"",
        path_display
    ));

    Ok(())
}

pub fn update(paths: Vec<String>, init: bool, recursive: bool, force: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut git_args = vec!["submodule", "update"];
    if init {
        git_args.push("--init");
    }
    if recursive {
        git_args.push("--recursive");
    }
    if force {
        git_args.push("--force");
    }

    let paths_ref: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    git_args.extend(paths_ref);

    let title = "Updating submodules";
    ProgressRunner::run(title, &git_args)?;

    let elapsed = start.elapsed();
    Ui::success_with_time("Submodules synchronized and checked out.", elapsed);
    Ok(())
}

pub fn initialize(paths: Vec<String>) -> HitResult<()> {
    let start = Instant::now();

    let mut git_args = vec!["submodule", "init"];
    let paths_ref: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    git_args.extend(paths_ref);

    let output = GitBackend::run(&git_args)?;

    let elapsed = start.elapsed();
    if output.stdout.trim().is_empty() {
        Ui::success_with_time("Submodules are already initialized.", elapsed);
    } else {
        for line in output.stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                println!("  {} {}", "✔".bright_green(), trimmed);
            }
        }
        Ui::success_with_time("Submodules initialized.", elapsed);
        Ui::hint("Run 'hit submodule update' to checkout recorded commits.");
    }

    Ok(())
}

pub fn sync(paths: Vec<String>, recursive: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut git_args = vec!["submodule", "sync"];
    if recursive {
        git_args.push("--recursive");
    }
    let paths_ref: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    git_args.extend(paths_ref);

    GitBackend::run(&git_args)?;

    let elapsed = start.elapsed();
    Ui::success_with_time(
        "Submodule remote URLs synchronized with .gitmodules.",
        elapsed,
    );
    Ok(())
}

pub fn deinit(args: Vec<String>, force: bool) -> HitResult<()> {
    let start = Instant::now();

    if args.is_empty() {
        return Err(HitError::with_hint(
            "Missing submodule path",
            "Specify the submodule directory path to deinitialize.",
            "Example:\n  hit submodule deinit vendor/lib\n  hit submodule deinit --force vendor/lib",
        ));
    }

    let path = &args[0];
    let mut git_args = vec!["submodule", "deinit"];
    if force {
        git_args.push("-f");
    }
    git_args.push(path.as_str());

    GitBackend::run(&git_args)?;

    let elapsed = start.elapsed();
    Ui::success_with_time(
        &format!("Deinitialized submodule '{}'.", path.bold()),
        elapsed,
    );
    Ok(())
}

pub fn foreach(args: Vec<String>) -> HitResult<()> {
    let start = Instant::now();

    if args.is_empty() {
        return Err(HitError::with_hint(
            "Missing command",
            "Provide a command to execute in each submodule directory.",
            "Example:\n  hit submodule foreach git status\n  hit submodule foreach pwd",
        ));
    }

    let command = args.join(" ");
    let git_args = ["submodule", "foreach", &command];
    let output = GitBackend::run(&git_args)?;

    for line in output.stdout.lines() {
        if line.starts_with("Entering '") {
            println!("{}", line.cyan().bold());
        } else {
            println!("  {}", line);
        }
    }

    let elapsed = start.elapsed();
    Ui::success_with_time("Completed submodule foreach command.", elapsed);
    Ok(())
}
