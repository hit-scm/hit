use crate::core::index::Index;
use crate::core::object::{Commit, ObjectType, Tree, TreeEntry};
use crate::core::storage::ObjectStore;
use crate::error::{HitError, HitResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HitConfig {
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub default_branch: Option<String>,
}

pub struct Repo {
    pub root: PathBuf,
}

#[allow(dead_code)]
impl Repo {
    /// Discover repository by walking up from the current working directory
    pub fn discover() -> HitResult<Self> {
        let current = std::env::current_dir()?;
        let mut dir = current.as_path();
        loop {
            if dir.join(".hit").is_dir() {
                return Ok(Self {
                    root: dir.to_path_buf(),
                });
            }
            match dir.parent() {
                Some(parent) => dir = parent,
                None => return Err(HitError::not_in_repo()),
            }
        }
    }

    /// Open repository from a known path
    pub fn open(path: impl AsRef<Path>) -> HitResult<Self> {
        let path = path.as_ref();
        if path.join(".hit").is_dir() {
            Ok(Self {
                root: path.to_path_buf(),
            })
        } else {
            Err(HitError::not_in_repo())
        }
    }

    pub fn hit_dir(&self) -> PathBuf {
        self.root.join(".hit")
    }

    pub fn head_path(&self) -> PathBuf {
        self.hit_dir().join("HEAD")
    }

    pub fn refs_heads_dir(&self) -> PathBuf {
        self.hit_dir().join("refs").join("heads")
    }

    pub fn refs_tags_dir(&self) -> PathBuf {
        self.hit_dir().join("refs").join("tags")
    }

    pub fn config_path(&self) -> PathBuf {
        self.hit_dir().join("config.json")
    }

    /// Read HEAD content
    pub fn read_head(&self) -> HitResult<String> {
        let p = self.head_path();
        if !p.exists() {
            return Err(HitError::new(
                "Repository corruption",
                "HEAD file is missing.",
            ));
        }
        let content = fs::read_to_string(p)?;
        Ok(content.trim().to_string())
    }

    /// Get current branch name, or None if in detached HEAD state
    pub fn current_branch(&self) -> HitResult<Option<String>> {
        let head = self.read_head()?;
        if let Some(branch_ref) = head.strip_prefix("ref: refs/heads/") {
            Ok(Some(branch_ref.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Get commit hash of current HEAD
    pub fn current_head_commit(&self) -> HitResult<Option<String>> {
        let head = self.read_head()?;
        if let Some(branch_ref) = head.strip_prefix("ref: refs/heads/") {
            let branch_file = self.refs_heads_dir().join(branch_ref);
            if branch_file.exists() {
                let hash = fs::read_to_string(branch_file)?;
                Ok(Some(hash.trim().to_string()))
            } else {
                Ok(None) // Brand new repository with no commits yet
            }
        } else {
            // Detached HEAD
            Ok(Some(head))
        }
    }

    /// Point HEAD to a branch
    pub fn set_head_branch(&self, branch: &str) -> HitResult<()> {
        let head_content = format!("ref: refs/heads/{}\n", branch);
        fs::write(self.head_path(), head_content)?;
        Ok(())
    }

    /// Point HEAD to a detached commit
    pub fn set_head_detached(&self, commit_hash: &str) -> HitResult<()> {
        fs::write(self.head_path(), format!("{}\n", commit_hash))?;
        Ok(())
    }

    /// Update or create branch ref with given commit hash
    pub fn set_branch_commit(&self, branch: &str, commit_hash: &str) -> HitResult<()> {
        let branch_path = self.refs_heads_dir().join(branch);
        if let Some(parent) = branch_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(branch_path, format!("{}\n", commit_hash))?;
        Ok(())
    }

    /// Read commit hash of a branch
    pub fn get_branch_commit(&self, branch: &str) -> HitResult<Option<String>> {
        let path = self.refs_heads_dir().join(branch);
        if path.exists() {
            let hash = fs::read_to_string(path)?;
            Ok(Some(hash.trim().to_string()))
        } else {
            Ok(None)
        }
    }

    /// List all local branches (including hierarchical ones like feature/foo)
    pub fn list_branches(&self) -> HitResult<Vec<String>> {
        let heads = self.refs_heads_dir();
        if !heads.exists() {
            return Ok(Vec::new());
        }

        let mut branches = Vec::new();
        for entry in walkdir::WalkDir::new(&heads)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(rel) = entry.path().strip_prefix(&heads) {
                    branches.push(rel.to_string_lossy().to_string());
                }
            }
        }
        branches.sort();
        Ok(branches)
    }

    /// Delete a branch
    pub fn delete_branch(&self, branch: &str) -> HitResult<()> {
        let path = self.refs_heads_dir().join(branch);
        if path.exists() {
            fs::remove_file(path)?;
            Ok(())
        } else {
            Err(HitError::branch_not_found(branch, None))
        }
    }

    /// List all tags
    pub fn list_tags(&self) -> HitResult<Vec<String>> {
        let tags_dir = self.refs_tags_dir();
        if !tags_dir.exists() {
            return Ok(Vec::new());
        }

        let mut tags = Vec::new();
        for entry in walkdir::WalkDir::new(&tags_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(rel) = entry.path().strip_prefix(&tags_dir) {
                    tags.push(rel.to_string_lossy().to_string());
                }
            }
        }
        tags.sort();
        Ok(tags)
    }

    /// Get commit or object pointed to by tag
    pub fn get_tag(&self, name: &str) -> HitResult<Option<String>> {
        let path = self.refs_tags_dir().join(name);
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(Some(content.trim().to_string()))
        } else {
            Ok(None)
        }
    }

    /// Set a tag
    pub fn set_tag(&self, name: &str, target_hash: &str) -> HitResult<()> {
        let tag_path = self.refs_tags_dir().join(name);
        if let Some(parent) = tag_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(tag_path, format!("{}\n", target_hash))?;
        Ok(())
    }

    /// Load repository configuration
    pub fn load_config(&self) -> HitConfig {
        let p = self.config_path();
        if p.exists() {
            if let Ok(data) = fs::read(&p) {
                if let Ok(cfg) = serde_json::from_slice(&data) {
                    return cfg;
                }
            }
        }
        HitConfig::default()
    }

    /// Save repository configuration
    pub fn save_config(&self, config: &HitConfig) -> HitResult<()> {
        let data = serde_json::to_vec_pretty(config).map_err(|e| {
            HitError::new(
                "Configuration error",
                format!("Failed to serialize config: {}", e),
            )
        })?;
        fs::write(self.config_path(), data)?;
        Ok(())
    }

    /// Resolve author identity (checks repo config, then global config, then env vars, then defaults)
    pub fn author_identity(&self) -> (String, String) {
        let local_cfg = self.load_config();
        let name = local_cfg
            .user_name
            .or_else(|| std::env::var("HIT_AUTHOR_NAME").ok())
            .or_else(|| std::env::var("GIT_AUTHOR_NAME").ok())
            .unwrap_or_else(|| "Hit Developer".to_string());

        let email = local_cfg
            .user_email
            .or_else(|| std::env::var("HIT_AUTHOR_EMAIL").ok())
            .or_else(|| std::env::var("GIT_AUTHOR_EMAIL").ok())
            .unwrap_or_else(|| "dev@hit-scm.com".to_string());

        (name, email)
    }

    /// Write hierarchical Tree objects from Index and return root Tree hash
    pub fn write_tree_from_index(&self, index: &Index) -> HitResult<String> {
        // Group entries into directory hierarchy
        // We'll create a nested structure: Directory node with files and subdirectories
        struct DirNode {
            files: Vec<(String, u32, String)>, // (name, mode, hash)
            subdirs: BTreeMap<String, DirNode>,
        }

        impl DirNode {
            fn new() -> Self {
                Self {
                    files: Vec::new(),
                    subdirs: BTreeMap::new(),
                }
            }

            fn insert(&mut self, path_parts: &[&str], mode: u32, hash: String) {
                if path_parts.len() == 1 {
                    self.files.push((path_parts[0].to_string(), mode, hash));
                } else {
                    let dir_name = path_parts[0];
                    let sub = self
                        .subdirs
                        .entry(dir_name.to_string())
                        .or_insert_with(DirNode::new);
                    sub.insert(&path_parts[1..], mode, hash);
                }
            }

            fn write_to_store(&self, repo_root: &Path) -> HitResult<String> {
                let mut entries = Vec::new();

                for (name, mode, hash) in &self.files {
                    entries.push(TreeEntry::new(*mode, ObjectType::Blob, hash, name));
                }

                for (name, subdir) in &self.subdirs {
                    let sub_hash = subdir.write_to_store(repo_root)?;
                    entries.push(TreeEntry::new(0o040000, ObjectType::Tree, sub_hash, name));
                }

                let tree = Tree::new(entries);
                let bytes = tree.serialize();
                ObjectStore::write_object(repo_root, ObjectType::Tree, &bytes)
            }
        }

        let mut root_node = DirNode::new();
        for (path, entry) in &index.entries {
            let parts: Vec<&str> = path.split('/').collect();
            root_node.insert(&parts, entry.mode, entry.hash.clone());
        }

        root_node.write_to_store(&self.root)
    }

    /// Read Tree recursively into flat map of `path -> (hash, mode)`
    pub fn read_tree_flat(&self, tree_hash: &str) -> HitResult<HashMap<String, (String, u32)>> {
        let mut map = HashMap::new();
        self.traverse_tree(tree_hash, "", &mut map)?;
        Ok(map)
    }

    fn traverse_tree(
        &self,
        tree_hash: &str,
        prefix: &str,
        acc: &mut HashMap<String, (String, u32)>,
    ) -> HitResult<()> {
        let (obj_type, data) = ObjectStore::read_object(&self.root, tree_hash)?;
        if obj_type != ObjectType::Tree {
            return Err(HitError::new(
                "Invalid object type",
                format!(
                    "Expected tree object for {}, found {:?}",
                    tree_hash, obj_type
                ),
            ));
        }

        let tree = Tree::deserialize(&data).map_err(|e| {
            HitError::new(
                "Corrupt tree object",
                format!("Failed to parse tree {}: {}", tree_hash, e),
            )
        })?;

        for entry in tree.entries {
            let current_path = if prefix.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", prefix, entry.name)
            };

            match entry.obj_type {
                ObjectType::Blob => {
                    acc.insert(current_path, (entry.hash, entry.mode));
                }
                ObjectType::Tree => {
                    self.traverse_tree(&entry.hash, &current_path, acc)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Read a commit object
    pub fn read_commit(&self, commit_hash: &str) -> HitResult<Commit> {
        let (obj_type, data) = ObjectStore::read_object(&self.root, commit_hash)?;
        if obj_type != ObjectType::Commit {
            return Err(HitError::new(
                "Invalid object",
                format!("Object {} is not a commit", commit_hash),
            ));
        }

        Commit::deserialize(&data).map_err(|e| {
            HitError::new(
                "Corrupt commit",
                format!("Failed to deserialize commit {}: {}", commit_hash, e),
            )
        })
    }
}
