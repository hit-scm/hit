use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use std::time::Instant;

pub fn execute(paths: Vec<String>, cached: bool) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["rm"];
    if cached {
        args.push("--cached");
    }

    for p in &paths {
        args.push(p.as_str());
    }

    GitBackend::run(&args)?;

    let elapsed = start.elapsed();
    let label = if cached {
        format!(
            "Unstaged {} path{}",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        )
    } else {
        format!(
            "Removed {} path{}",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        )
    };

    Ui::success_with_time(&label, elapsed);
    Ok(())
}
