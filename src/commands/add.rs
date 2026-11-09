use crate::error::HitResult;
use crate::ui::Ui;
use std::time::Instant;

pub fn execute(paths: Vec<String>) -> HitResult<()> {
    let start = Instant::now();

    let targets = if paths.is_empty() {
        vec![".".to_string()]
    } else {
        paths
    };

    let mut args = vec!["add"];
    for p in &targets {
        args.push(p.as_str());
    }

    crate::ui::ProgressRunner::run_raw("Staging changes", &args)?;

    let elapsed = start.elapsed();
    let label = if targets.len() == 1 && targets[0] == "." {
        "Staged all changes".to_string()
    } else {
        format!(
            "Staged {} path{}",
            targets.len(),
            if targets.len() == 1 { "" } else { "s" }
        )
    };

    Ui::success_with_time(&label, elapsed);
    Ok(())
}
