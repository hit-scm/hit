use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(short: bool) -> HitResult<()> {
    let start = Instant::now();

    if short {
        let output = GitBackend::run(&["status", "--short"])?;
        if !output.stdout.trim().is_empty() {
            for line in output.stdout.lines() {
                if line.starts_with("??") {
                    println!("{}", line.bright_cyan());
                } else if line.starts_with('M') || line.starts_with(" M") {
                    println!("{}", line.bright_yellow());
                } else if line.starts_with('A') || line.starts_with(" A") {
                    println!("{}", line.bright_green());
                } else if line.starts_with('D') || line.starts_with(" D") {
                    println!("{}", line.bright_red());
                } else {
                    println!("{}", line);
                }
            }
        }
        println!("{}", Ui::format_duration(start.elapsed()));
        return Ok(());
    }

    let output = GitBackend::run(&["status", "--porcelain=v2", "--branch"])?;

    let mut branch = String::new();
    let mut upstream = String::new();
    let mut ahead = 0;
    let mut behind = 0;
    let mut initial_commit = false;

    let mut staged_new = Vec::new();
    let mut staged_mod = Vec::new();
    let mut staged_del = Vec::new();

    let mut unstaged_mod = Vec::new();
    let mut unstaged_del = Vec::new();

    let mut untracked = Vec::new();
    let mut unmerged = Vec::new();

    for line in output.stdout.lines() {
        if let Some(rest) = line.strip_prefix("# branch.head ") {
            branch = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# branch.oid ") {
            if rest.contains("(initial)") {
                initial_commit = true;
            }
        } else if let Some(rest) = line.strip_prefix("# branch.upstream ") {
            upstream = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# branch.ab ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                ahead = parts[0].trim_start_matches('+').parse().unwrap_or(0);
                behind = parts[1].trim_start_matches('-').parse().unwrap_or(0);
            }
        } else if line.starts_with("1 ") || line.starts_with("2 ") {
            // Ordinary or renamed changed entry
            // 1 <XY> <sub> <mH> <mI> <mW> <hH> <hI> <path>
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                let xy = parts[1];
                let path = parts[8..].join(" ");
                let x = xy.chars().next().unwrap_or('.');
                let y = xy.chars().nth(1).unwrap_or('.');

                match x {
                    'A' => staged_new.push(path.clone()),
                    'M' | 'R' => staged_mod.push(path.clone()),
                    'D' => staged_del.push(path.clone()),
                    _ => {}
                }

                match y {
                    'M' => unstaged_mod.push(path),
                    'D' => unstaged_del.push(path),
                    _ => {}
                }
            }
        } else if let Some(path) = line.strip_prefix("? ") {
            untracked.push(path.trim().to_string());
        } else if line.starts_with("u ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                unmerged.push(parts[10..].join(" "));
            }
        }
    }

    // Print branch header
    if branch == "(detached)" {
        println!("HEAD detached");
    } else {
        println!("On branch {}", branch.bold().bright_green());
    }

    if initial_commit {
        println!("{}", "No commits yet".bright_black());
    }

    if !upstream.is_empty() {
        if ahead > 0 && behind > 0 {
            println!(
                "Your branch and '{}' have diverged, and have {} and {} different commits each.",
                upstream.cyan(),
                ahead,
                behind
            );
        } else if ahead > 0 {
            println!(
                "Your branch is ahead of '{}' by {} commit{} (use \"hit push\" to publish).",
                upstream.cyan(),
                ahead,
                if ahead == 1 { "" } else { "s" }
            );
        } else if behind > 0 {
            println!(
                "Your branch is behind '{}' by {} commit{} (use \"hit pull\" to update).",
                upstream.cyan(),
                behind,
                if behind == 1 { "" } else { "s" }
            );
        }
    }
    println!();

    // Unmerged conflicts
    if !unmerged.is_empty() {
        println!(
            "{}",
            "Unmerged paths (fix conflicts and run \"hit commit\"):"
                .bold()
                .bright_red()
        );
        for p in &unmerged {
            println!(
                "        {}   {}",
                "both modified:".bright_red(),
                p.bright_red()
            );
        }
        println!();
    }

    // Staged changes
    let has_staged = !staged_new.is_empty() || !staged_mod.is_empty() || !staged_del.is_empty();
    if has_staged {
        println!("{}", "Changes to be committed:".bold().white());
        println!("  {}", Ui::dim("(use \"hit reset <file>...\" to unstage)"));
        for f in &staged_new {
            println!(
                "        {}   {}",
                "new file:".bright_green(),
                f.bright_green()
            );
        }
        for f in &staged_mod {
            println!(
                "        {}   {}",
                "modified:".bright_green(),
                f.bright_green()
            );
        }
        for f in &staged_del {
            println!(
                "        {}   {}",
                "deleted: ".bright_green(),
                f.bright_green()
            );
        }
        println!();
    }

    // Unstaged changes
    let has_unstaged = !unstaged_mod.is_empty() || !unstaged_del.is_empty();
    if has_unstaged {
        println!("{}", "Changes not staged for commit:".bold().white());
        println!(
            "  {}",
            Ui::dim("(use \"hit add <file>...\" to update what will be committed)")
        );
        println!(
            "  {}",
            Ui::dim("(use \"hit checkout -- <file>...\" to discard changes in working directory)")
        );
        for f in &unstaged_mod {
            println!(
                "        {}   {}",
                "modified:".bright_yellow(),
                f.bright_yellow()
            );
        }
        for f in &unstaged_del {
            println!("        {}   {}", "deleted: ".bright_red(), f.bright_red());
        }
        println!();
    }

    // Untracked files
    if !untracked.is_empty() {
        println!("{}", "Untracked files:".bold().white());
        println!(
            "  {}",
            Ui::dim("(use \"hit add <file>...\" to include in what will be committed)")
        );
        for f in untracked.iter().take(25) {
            println!("        {}", f.bright_cyan());
        }
        if untracked.len() > 25 {
            println!(
                "        {}",
                format!("... and {} more files", untracked.len() - 25).bright_black()
            );
        }
        println!();
    }

    let elapsed = start.elapsed();
    if !has_staged && !has_unstaged && untracked.is_empty() && unmerged.is_empty() {
        println!(
            "{} {}",
            "✔ Nothing to commit, working tree clean"
                .bright_green()
                .bold(),
            Ui::format_duration(elapsed)
        );
        if ahead > 0 {
            Ui::hint("Publish local commits with: hit push");
        } else if behind > 0 {
            Ui::hint("Update from upstream with: hit pull");
        }
    } else {
        println!("{}", Ui::format_duration(elapsed));
        if has_staged && !has_unstaged && untracked.is_empty() {
            Ui::hint("Ready to commit: hit commit -m \"Your message\"");
        } else if has_unstaged || !untracked.is_empty() {
            Ui::hint("Stage changes with 'hit add <file>...' or stage all with 'hit add .'");
        }
    }

    if std::path::Path::new(".gitmodules").exists() {
        if let Ok(sub_out) = GitBackend::run(&["submodule", "status"]) {
            let uninit_count = sub_out
                .stdout
                .lines()
                .filter(|l| l.trim_start().starts_with('-'))
                .count();
            if uninit_count > 0 {
                println!(
                    "{} {} uninitialized submodule(s) detected",
                    "ℹ".bright_yellow(),
                    uninit_count
                );
                Ui::hint("Initialize submodules with: hit submodule update --init --recursive");
            }
        }
    }

    Ok(())
}
