use crate::error::HitResult;
use crate::git::GitBackend;
use crate::ui::Ui;
use std::fs;
use std::time::Instant;

pub fn execute(target_path: Option<&str>, default_branch: Option<&str>) -> HitResult<()> {
    let start = Instant::now();
    let branch = default_branch.unwrap_or("main");

    let mut args = vec!["init", "-b", branch];
    if let Some(p) = target_path {
        args.push(p);
    }

    GitBackend::run(&args)?;

    let current_dir = std::env::current_dir()?;
    let repo_root = match target_path {
        Some(p) => current_dir.join(p),
        None => current_dir,
    };

    // Create starter .gitignore / .hitignore if none exists
    let gitignore_path = repo_root.join(".gitignore");
    if !gitignore_path.exists() {
        let starter_ignore = "# Dependencies & Build Outputs\ntarget/\nnode_modules/\ndist/\n\n# System files\n.DS_Store\nThumbs.db\n*.log\n";
        let _ = fs::write(&gitignore_path, starter_ignore);
    }

    let hitignore_path = repo_root.join(".hitignore");
    if !hitignore_path.exists() {
        let _ = fs::write(hitignore_path, "# Hit Ignore (inherits from .gitignore)\n");
    }

    let elapsed = start.elapsed();
    Ui::success_with_time(
        &format!(
            "Initialized empty Hit repository in {}",
            repo_root.display()
        ),
        elapsed,
    );
    Ui::hint(
        "Next: Add files with 'hit add .' and make your first commit with 'hit commit -m \"...\"'",
    );

    Ok(())
}
