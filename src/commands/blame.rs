use crate::error::{HitError, HitResult};
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(file: String, lines_range: Option<String>) -> HitResult<()> {
    let start = Instant::now();

    if file.trim().is_empty() {
        return Err(HitError::with_hint(
            "Missing file argument for blame",
            "Specify the file path you wish to inspect.",
            "Example:\n  hit blame src/main.rs",
        ));
    }

    let mut args = vec!["blame"];

    let range_flag;
    if let Some(ref r) = lines_range {
        range_flag = format!("-L{}", r);
        args.push(range_flag.as_str());
    }

    args.push(file.as_str());

    let output = GitBackend::run(&args)?;

    for line in output.stdout.lines() {
        if line.len() > 40 {
            let hash = &line[0..8];
            let rest = &line[8..];
            if let (Some(open_paren), Some(close_paren)) = (rest.find('('), rest.find(')')) {
                let author_date = &rest[open_paren + 1..close_paren];
                let code = &rest[close_paren + 1..];
                println!(
                    "{} {} {}",
                    hash.bright_yellow(),
                    format!("({})", author_date).bright_black(),
                    code
                );
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
