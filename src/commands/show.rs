use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(object_ref: Option<String>) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["show"];
    if let Some(ref o) = object_ref {
        args.push(o.as_str());
    }

    let output = GitBackend::run(&args)?;

    for line in output.stdout.lines() {
        if line.starts_with("commit ") {
            println!("{}", line.bright_yellow().bold());
        } else if line.starts_with("Author:") {
            println!("{}", line);
        } else if line.starts_with("Date:") {
            println!("{}", line.bright_black());
        } else if line.starts_with("diff --") {
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
        } else {
            println!("{}", line);
        }
    }

    println!("{}", Ui::format_duration(start.elapsed()));
    Ok(())
}
