use crate::engine::repo::RepoEngine;
use std::fs;
use walkdir::WalkDir;

pub struct BranchEngine;

impl BranchEngine {
    pub fn current_branch() -> Option<String> {
        let root = RepoEngine::find_root()?;
        let git_dir = RepoEngine::git_dir(&root);
        let head_file = git_dir.join("HEAD");

        if let Ok(content) = fs::read_to_string(head_file) {
            let trimmed = content.trim();
            if let Some(branch_ref) = trimmed.strip_prefix("ref: refs/heads/") {
                return Some(branch_ref.to_string());
            }
        }
        None
    }

    pub fn list_local_branches() -> Vec<String> {
        let mut branches = Vec::new();
        if let Some(root) = RepoEngine::find_root() {
            let git_dir = RepoEngine::git_dir(&root);
            let heads_dir = git_dir.join("refs").join("heads");

            if heads_dir.exists() {
                for entry in WalkDir::new(&heads_dir).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Ok(rel) = entry.path().strip_prefix(&heads_dir) {
                            branches.push(rel.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        branches.sort();
        branches
    }

    pub fn get_ref_sha(ref_path: &str) -> Option<String> {
        let root = RepoEngine::find_root()?;
        let git_dir = RepoEngine::git_dir(&root);

        // 1. Check loose ref file
        let direct_file = git_dir.join(ref_path);
        if let Ok(content) = fs::read_to_string(&direct_file) {
            let trimmed = content.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("ref:") {
                return Some(trimmed.to_string());
            }
        }

        // 2. Check packed-refs file
        let packed_file = git_dir.join("packed-refs");
        if let Ok(content) = fs::read_to_string(&packed_file) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('#') || trimmed.starts_with('^') {
                    continue;
                }
                let mut parts = trimmed.split_whitespace();
                if let (Some(sha), Some(name)) = (parts.next(), parts.next()) {
                    if name == ref_path {
                        return Some(sha.to_string());
                    }
                }
            }
        }

        None
    }

    pub fn is_up_to_date_with_remote(branch: &str, remote: &str) -> bool {
        let local_ref = format!("refs/heads/{}", branch);
        let remote_ref = format!("refs/remotes/{}/{}", remote, branch);

        if let (Some(local_sha), Some(remote_sha)) = (
            Self::get_ref_sha(&local_ref),
            Self::get_ref_sha(&remote_ref),
        ) {
            local_sha == remote_sha
        } else {
            false
        }
    }
}
