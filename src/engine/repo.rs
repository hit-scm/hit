use std::path::{Path, PathBuf};

pub struct RepoEngine;

impl RepoEngine {
    pub fn find_root() -> Option<PathBuf> {
        let current = std::env::current_dir().ok()?;
        let mut dir = current.as_path();
        loop {
            if dir.join(".git").exists() || dir.join(".hit").is_dir() {
                return Some(dir.to_path_buf());
            }
            dir = dir.parent()?;
        }
    }

    pub fn is_in_repo() -> bool {
        Self::find_root().is_some()
    }

    pub fn git_dir(root: &Path) -> PathBuf {
        let git_path = root.join(".git");
        if git_path.is_file() {
            // Handle git worktree or submodule pointer
            if let Ok(content) = std::fs::read_to_string(&git_path) {
                if let Some(rest) = content.strip_prefix("gitdir: ") {
                    let rel = rest.trim();
                    let target = Path::new(rel);
                    if target.is_absolute() {
                        return target.to_path_buf();
                    } else {
                        return root.join(target);
                    }
                }
            }
        }
        git_path
    }
}
