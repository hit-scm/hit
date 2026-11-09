use crate::engine::repo::RepoEngine;
use crate::error::{HitError, HitResult};
use std::path::PathBuf;
use std::process::Command;

pub struct HookRunner;

impl HookRunner {
    pub fn find_hook(name: &str) -> Option<PathBuf> {
        let root = RepoEngine::find_root()?;
        let git_hook = RepoEngine::git_dir(&root).join("hooks").join(name);
        if git_hook.is_file() {
            return Some(git_hook);
        }

        let hit_hook = root.join(".hit").join("hooks").join(name);
        if hit_hook.is_file() {
            return Some(hit_hook);
        }

        None
    }

    pub fn execute_hook(name: &str, args: &[&str]) -> HitResult<()> {
        if let Some(hook_path) = Self::find_hook(name) {
            let status = Command::new(&hook_path).args(args).status().map_err(|e| {
                HitError::with_hint(
                    format!("Failed to execute hook '{}'", name),
                    format!(
                        "Hook script at '{}' failed to execute: {}",
                        hook_path.display(),
                        e
                    ),
                    "Verify hook file permissions (chmod +x) and interpreter shebang.",
                )
            })?;

            if !status.success() {
                return Err(HitError::with_hint(
                    format!("Hook '{}' rejected this operation", name),
                    format!("The {} hook exited with non-zero status code.", name),
                    "Resolve the issues reported by the hook above, or bypass with '--no-verify' if applicable.",
                ));
            }
        }
        Ok(())
    }
}
