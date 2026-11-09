use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use std::time::Instant;

pub fn execute(
    paths: Vec<String>,
    hard: bool,
    soft: bool,
    commit_target: Option<String>,
) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["reset"];

    if hard {
        args.push("--hard");
    } else if soft {
        args.push("--soft");
    }

    if let Some(ref c) = commit_target {
        args.push(c.as_str());
    }

    if !paths.is_empty() {
        args.push("--");
        for p in &paths {
            args.push(p.as_str());
        }
    }

    GitBackend::run(&args)?;

    let elapsed = start.elapsed();
    let label = if !paths.is_empty() {
        format!(
            "Unstaged {} path{}",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        )
    } else if hard {
        "Reset working tree and index (hard reset)".to_string()
    } else if soft {
        "Reset HEAD reference (soft reset)".to_string()
    } else {
        "Unstaged changes (mixed reset)".to_string()
    };

    Ui::success_with_time(&label, elapsed);
    Ok(())
}
