use crate::error::HitResult;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    pub hash: String,
    pub size: u64,
    pub mtime: i64,
    pub mode: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Index {
    pub entries: BTreeMap<String, IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn index_file(repo_root: &Path) -> std::path::PathBuf {
        repo_root.join(".hit").join("index")
    }

    pub fn load(repo_root: &Path) -> HitResult<Self> {
        let path = Self::index_file(repo_root);
        if !path.exists() {
            return Ok(Self::new());
        }

        let bytes = fs::read(&path)?;
        if bytes.is_empty() {
            return Ok(Self::new());
        }

        let index: Index = serde_json::from_slice(&bytes).unwrap_or_else(|_| Index::new());
        Ok(index)
    }

    pub fn save(&self, repo_root: &Path) -> HitResult<()> {
        let path = Self::index_file(repo_root);
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| {
            crate::error::HitError::new("Failed to serialize index", format!("JSON error: {}", e))
        })?;
        fs::write(path, bytes)?;
        Ok(())
    }

    pub fn add(&mut self, path: String, hash: String, size: u64, mtime: i64, mode: u32) {
        self.entries.insert(
            path,
            IndexEntry {
                hash,
                size,
                mtime,
                mode,
            },
        );
    }

    pub fn contains_key(&self, path: &str) -> bool {
        self.entries.contains_key(path)
    }

    pub fn remove(&mut self, path: &str) -> bool {
        self.entries.remove(path).is_some()
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
