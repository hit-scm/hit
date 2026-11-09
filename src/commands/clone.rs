use crate::error::HitResult;
use crate::ui::Ui;
use colored::*;
use std::path::Path;
use std::time::Instant;

pub fn execute(
    url: String,
    target_path: Option<String>,
    depth: Option<usize>,
    branch: Option<String>,
    recurse_submodules: bool,
) -> HitResult<()> {
    let start = Instant::now();

    let mut args = vec!["clone", url.as_str()];

    if recurse_submodules {
        args.push("--recurse-submodules");
    }

    let depth_str = depth.map(|d| d.to_string());
    if let Some(ref d) = depth_str {
        args.push("--depth");
        args.push(d.as_str());
    }

    if let Some(ref b) = branch {
        args.push("--branch");
        args.push(b.as_str());
    }

    if let Some(ref p) = target_path {
        args.push(p.as_str());
    }

    // Determine target directory name for reporting
    let dir_name = match &target_path {
        Some(p) => p.clone(),
        None => {
            // Derive directory from URL (e.g. https://github.com/user/repo.git -> repo)
            let trimmed = url.trim_end_matches('/').trim_end_matches(".git");
            trimmed.rsplit('/').next().unwrap_or("repo").to_string()
        }
    };

    let title = format!("Cloning {}", dir_name);
    crate::ui::ProgressRunner::run(&title, &args)?;

    let elapsed = start.elapsed();
    Ui::success_with_time(
        &format!("Cloned {} into ./{}", url.cyan(), dir_name.bold()),
        elapsed,
    );

    if Path::new(&dir_name).exists() {
        if Path::new(&dir_name).join(".gitmodules").exists() && !recurse_submodules {
            Ui::hint(&format!(
                "Repository contains submodules. Initialize them with:\n  cd {} && hit submodule update --init --recursive",
                dir_name
            ));
        } else {
            Ui::hint(&format!("Next: cd {} && hit status", dir_name));
        }
    }

    Ok(())
}
