use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(staged: bool, stat: bool, path_filter: Option<String>) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["diff"];

    if staged {
        args.push("--staged");
    }

    if stat {
        args.push("--stat");
    }

    if let Some(ref p) = path_filter {
        args.push("--");
        args.push(p.as_str());
    }

    let output = GitBackend::run(&args)?;

    if output.stdout.trim().is_empty() {
        println!(
            "{} {}",
            "No differences found".bright_black(),
            Ui::format_duration(start.elapsed())
        );
        return Ok(());
    }

    for line in output.stdout.lines() {
        if line.starts_with("diff --") {
            println!("{}", line.bold().white());
        } else if line.starts_with("--- ") {
            println!("{}", line.bold().bright_red());
        } else if line.starts_with("+++ ") {
            println!("{}", line.bold().bright_green());
        } else if line.starts_with("@@ ") {
            println!("{}", line.bright_cyan());
        } else if line.starts_with('+') {
            println!("{}", line.bright_green());
        } else if line.starts_with('-') {
            println!("{}", line.bright_red());
        } else if line.contains("changed")
            || line.contains("insertion")
            || line.contains("deletion")
        {
            println!("{}", crate::ui::diff_stats::colorize_diff_summary(line));
        } else if line.contains('|') && (line.contains('+') || line.contains('-')) {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                let file_part = parts[0];
                let rest = parts[1];
                let mut colored_rest = String::new();
                for c in rest.chars() {
                    if c == '+' {
                        colored_rest.push_str(&"+".bright_green().to_string());
                    } else if c == '-' {
                        colored_rest.push_str(&"-".bright_red().to_string());
                    } else {
                        colored_rest.push(c);
                    }
                }
                println!("{}|{}", file_part, colored_rest);
            } else {
                println!("{}", line);
            }
        } else {
            println!("{}", line);
        }
    }

    println!("{}", Ui::format_duration(start.elapsed()));
    Ok(())
}
