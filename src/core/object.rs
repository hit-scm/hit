use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tag => write!(f, "tag"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeEntry {
    pub mode: u32,
    pub obj_type: ObjectType,
    pub hash: String,
    pub name: String,
}

impl TreeEntry {
    pub fn new(
        mode: u32,
        obj_type: ObjectType,
        hash: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            obj_type,
            hash: hash.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new(mut entries: Vec<TreeEntry>) -> Self {
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Self { entries }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = String::new();
        for entry in &self.entries {
            out.push_str(&format!(
                "{:o} {} {} {}\n",
                entry.mode, entry.obj_type, entry.hash, entry.name
            ));
        }
        out.into_bytes()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            if parts.len() < 4 {
                return Err("Corrupted tree entry".to_string());
            }
            let mode = u32::from_str_radix(parts[0], 8).map_err(|e| e.to_string())?;
            let obj_type = match parts[1] {
                "blob" => ObjectType::Blob,
                "tree" => ObjectType::Tree,
                "commit" => ObjectType::Commit,
                "tag" => ObjectType::Tag,
                _ => return Err(format!("Unknown object type {}", parts[1])),
            };
            let hash = parts[2].to_string();
            let name = parts[3].to_string();
            entries.push(TreeEntry {
                mode,
                obj_type,
                hash,
                name,
            });
        }
        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commit {
    pub tree: String,
    pub parents: Vec<String>,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub message: String,
}

impl Commit {
    pub fn new(
        tree: impl Into<String>,
        parents: Vec<String>,
        author_name: impl Into<String>,
        author_email: impl Into<String>,
        timestamp: i64,
        message: impl Into<String>,
    ) -> Self {
        Self {
            tree: tree.into(),
            parents,
            author_name: author_name.into(),
            author_email: author_email.into(),
            timestamp,
            message: message.into(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&format!("tree {}\n", self.tree));
        for parent in &self.parents {
            out.push_str(&format!("parent {}\n", parent));
        }
        out.push_str(&format!(
            "author {} <{}> {}\n",
            self.author_name, self.author_email, self.timestamp
        ));
        out.push('\n');
        out.push_str(&self.message);
        out.into_bytes()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let mut tree = String::new();
        let mut parents = Vec::new();
        let mut author_name = String::new();
        let mut author_email = String::new();
        let mut timestamp = 0i64;

        let mut lines = text.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some(t) = line.strip_prefix("tree ") {
                tree = t.trim().to_string();
            } else if let Some(p) = line.strip_prefix("parent ") {
                parents.push(p.trim().to_string());
            } else if let Some(a) = line.strip_prefix("author ") {
                // author Name <email> timestamp
                if let (Some(lt), Some(gt)) = (a.find('<'), a.rfind('>')) {
                    author_name = a[..lt].trim().to_string();
                    author_email = a[lt + 1..gt].trim().to_string();
                    let rest = a[gt + 1..].trim();
                    timestamp = rest.parse().unwrap_or(0);
                }
            }
        }

        let message = lines.collect::<Vec<&str>>().join("\n");

        Ok(Self {
            tree,
            parents,
            author_name,
            author_email,
            timestamp,
            message,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub target: String,
    pub name: String,
    pub tagger_name: String,
    pub tagger_email: String,
    pub timestamp: i64,
    pub message: String,
}

#[allow(dead_code)]
impl Tag {
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&format!("object {}\n", self.target));
        out.push_str("type commit\n");
        out.push_str(&format!("tag {}\n", self.name));
        out.push_str(&format!(
            "tagger {} <{}> {}\n",
            self.tagger_name, self.tagger_email, self.timestamp
        ));
        out.push('\n');
        out.push_str(&self.message);
        out.into_bytes()
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let mut target = String::new();
        let mut name = String::new();
        let mut tagger_name = String::new();
        let mut tagger_email = String::new();
        let mut timestamp = 0i64;

        let mut lines = text.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some(o) = line.strip_prefix("object ") {
                target = o.trim().to_string();
            } else if let Some(t) = line.strip_prefix("tag ") {
                name = t.trim().to_string();
            } else if let Some(a) = line.strip_prefix("tagger ") {
                if let (Some(lt), Some(gt)) = (a.find('<'), a.rfind('>')) {
                    tagger_name = a[..lt].trim().to_string();
                    tagger_email = a[lt + 1..gt].trim().to_string();
                    let rest = a[gt + 1..].trim();
                    timestamp = rest.parse().unwrap_or(0);
                }
            }
        }

        let message = lines.collect::<Vec<&str>>().join("\n");

        Ok(Self {
            target,
            name,
            tagger_name,
            tagger_email,
            timestamp,
            message,
        })
    }
}
