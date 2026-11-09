use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute(
    name: Option<String>,
    message: Option<String>,
    delete: Option<String>,
) -> HitResult<()> {
    let start = Instant::now();

    if let Some(del_name) = delete {
        GitBackend::run(&["tag", "-d", &del_name])?;
        Ui::success_with_time(&format!("Deleted tag '{}'", del_name), start.elapsed());
        return Ok(());
    }

    if let Some(tag_name) = name {
        let mut args = vec!["tag"];
        if let Some(ref m) = message {
            args.push("-a");
            args.push("-m");
            args.push(m.as_str());
        }
        args.push(tag_name.as_str());

        GitBackend::run(&args)?;
        Ui::success_with_time(
            &format!("Created tag '{}'", tag_name.bright_magenta()),
            start.elapsed(),
        );
        return Ok(());
    }

    let output = GitBackend::run(&["tag", "-l"])?;
    if output.stdout.trim().is_empty() {
        println!("{}", "No tags yet".bright_black());
    } else {
        for t in output.stdout.lines() {
            println!("{}", t.bright_magenta());
        }
    }

    println!("{}", Ui::format_duration(start.elapsed()));
    Ok(())
}
