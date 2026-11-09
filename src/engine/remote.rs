use crate::engine::repo::RepoEngine;
use std::fs;

pub struct RemoteEngine;

impl RemoteEngine {
    pub fn list_remotes() -> Vec<(String, String)> {
        let mut remotes = Vec::new();

        if let Some(root) = RepoEngine::find_root() {
            let git_dir = RepoEngine::git_dir(&root);
            let config_path = git_dir.join("config");

            if let Ok(content) = fs::read_to_string(config_path) {
                let mut current_remote = None;
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("[remote \"") && trimmed.ends_with("\"]") {
                        let name = &trimmed[9..trimmed.len() - 2];
                        current_remote = Some(name.to_string());
                    } else if let Some(ref r_name) = current_remote {
                        if let Some(stripped) = trimmed.strip_prefix("url = ") {
                            let url = stripped.trim().to_string();
                            remotes.push((r_name.clone(), url));
                            current_remote = None;
                        }
                    }
                }
            }
        }

        remotes
    }
}
