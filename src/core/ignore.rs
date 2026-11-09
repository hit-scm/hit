use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub struct IgnoreManager {
    matcher: Gitignore,
}

impl IgnoreManager {
    pub fn new(repo_root: &Path) -> Self {
        let mut builder = GitignoreBuilder::new(repo_root);

        // Always ignore internal directories
        let _ = builder.add_line(None, ".hit/");
        let _ = builder.add_line(None, ".git/");

        // Check for .hitignore
        let hitignore = repo_root.join(".hitignore");
        if hitignore.exists() {
            let _ = builder.add(&hitignore);
        }

        // Check for .gitignore as fallback/supplement
        let gitignore = repo_root.join(".gitignore");
        if gitignore.exists() {
            let _ = builder.add(&gitignore);
        }

        let matcher = builder
            .build()
            .unwrap_or_else(|_| GitignoreBuilder::new(repo_root).build().unwrap());

        Self { matcher }
    }

    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        self.matcher.matched(path, is_dir).is_ignore()
    }
}
