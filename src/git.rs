use crate::engine::{BranchEngine, EngineRunner, GitOutput, RemoteEngine, RepoEngine};
use crate::error::HitError;

pub struct GitBackend;

impl GitBackend {
    #[inline]
    pub fn run(args: &[&str]) -> Result<GitOutput, HitError> {
        EngineRunner::run(args)
    }

    #[inline]
    pub fn is_in_repo() -> bool {
        RepoEngine::is_in_repo()
    }

    #[inline]
    pub fn current_branch() -> Option<String> {
        BranchEngine::current_branch()
    }

    #[inline]
    pub fn list_local_branches() -> Vec<String> {
        BranchEngine::list_local_branches()
    }

    #[inline]
    pub fn list_remotes() -> Vec<(String, String)> {
        RemoteEngine::list_remotes()
    }

    #[inline]
    pub fn is_up_to_date_with_remote(branch: &str, remote: &str) -> bool {
        BranchEngine::is_up_to_date_with_remote(branch, remote)
    }

    #[inline]
    pub fn get_ref_sha(ref_path: &str) -> Option<String> {
        BranchEngine::get_ref_sha(ref_path)
    }
}
