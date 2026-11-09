use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(oneline: bool, limit: Option<usize>, graph: bool, stat: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["log", "--decorate"];

    if oneline {
        args.push("--oneline");
    }

    if graph {
        args.push("--graph");
    }

    if stat {
        args.push("--stat");
    }

    let limit_str = limit.map(|n| format!("-{}", n));
    if let Some(ref l) = limit_str {
        args.push(l.as_str());
    }

    let output = GitBackend::run(&args)?;

    for line in output.stdout.lines() {
        if line.starts_with("commit ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            let hash_and_decor = parts.get(1).unwrap_or(&"");
            println!("{} {}", "commit".bright_yellow().bold(), hash_and_decor);
        } else if line.starts_with("Author:") {
            println!("{}", line);
        } else if line.starts_with("Date:") {
            println!("{}", line.bright_black());
        } else if oneline {
            // In oneline, color the hash
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                println!("{} {}", parts[0].bright_yellow().bold(), parts[1]);
            } else {
                println!("{}", line);
            }
        } else if line.contains("changed")
            || line.contains("insertion")
            || line.contains("deletion")
        {
            println!("{}", crate::ui::diff_stats::colorize_diff_summary(line));
        } else if line.contains('|') && (line.contains('+') || line.contains('-')) {
            // Stat line e.g. "src/main.rs | 5 +++--"
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
