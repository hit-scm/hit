use crate::core::hash;
use crate::core::object::ObjectType;
use crate::error::{HitError, HitResult};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ObjectStore;

impl ObjectStore {
    pub fn objects_dir(repo_root: &Path) -> PathBuf {
        repo_root.join(".hit").join("objects")
    }

    /// Compute the object hash for a blob without writing it
    pub fn blob_hash(data: &[u8]) -> String {
        let header = format!("blob {}\0", data.len());
        let mut full_payload = Vec::with_capacity(header.len() + data.len());
        full_payload.extend_from_slice(header.as_bytes());
        full_payload.extend_from_slice(data);
        hash::hash_bytes(&full_payload)
    }

    /// Store raw object data with header `<type> <size>\0<data>`, compressed with zstd
    pub fn write_object(repo_root: &Path, obj_type: ObjectType, data: &[u8]) -> HitResult<String> {
        let header = format!("{} {}\0", obj_type, data.len());
        let mut full_payload = Vec::with_capacity(header.len() + data.len());
        full_payload.extend_from_slice(header.as_bytes());
        full_payload.extend_from_slice(data);

        let hash = hash::hash_bytes(&full_payload);
        let prefix = &hash[0..2];
        let suffix = &hash[2..];

        let dir = Self::objects_dir(repo_root).join(prefix);
        fs::create_dir_all(&dir)?;

        let obj_path = dir.join(suffix);
        if !obj_path.exists() {
            let compressed = zstd::encode_all(&full_payload[..], 3).map_err(|e| {
                HitError::new(
                    "Compression error",
                    format!("Failed to compress object {}: {}", hash, e),
                )
            })?;

            // Write atomically to avoid partial reads
            let tmp_path = dir.join(format!(".tmp_{}", suffix));
            fs::write(&tmp_path, compressed)?;
            fs::rename(tmp_path, obj_path)?;
        }

        Ok(hash)
    }

    /// Read an object by full or abbreviated hash
    pub fn read_object(repo_root: &Path, hash_query: &str) -> HitResult<(ObjectType, Vec<u8>)> {
        let full_hash = Self::resolve_hash(repo_root, hash_query)?;
        let prefix = &full_hash[0..2];
        let suffix = &full_hash[2..];

        let obj_path = Self::objects_dir(repo_root).join(prefix).join(suffix);
        if !obj_path.exists() {
            return Err(HitError::with_hint(
                format!("Object not found: {}", hash_query),
                format!(
                    "The object with hash '{}' does not exist in the repository.",
                    hash_query
                ),
                "Verify the commit, tag, or tree hash with 'hit log' or 'hit status'.",
            ));
        }

        let compressed = fs::read(&obj_path)?;
        let decompressed = zstd::decode_all(&compressed[..]).map_err(|e| {
            HitError::new(
                "Decompression error",
                format!("Failed to decompress object {}: {}", full_hash, e),
            )
        })?;

        // Find \0 delimiter separating header and body
        let null_pos = decompressed
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| HitError::new("Corrupt object", "Object header missing delimiter."))?;

        let header_str = std::str::from_utf8(&decompressed[..null_pos])
            .map_err(|_| HitError::new("Corrupt object", "Invalid UTF-8 in object header."))?;

        let mut parts = header_str.split(' ');
        let type_str = parts
            .next()
            .ok_or_else(|| HitError::new("Corrupt object", "Object type missing in header."))?;

        let obj_type = match type_str {
            "blob" => ObjectType::Blob,
            "tree" => ObjectType::Tree,
            "commit" => ObjectType::Commit,
            "tag" => ObjectType::Tag,
            _ => {
                return Err(HitError::new(
                    "Corrupt object",
                    format!("Unknown object type: {}", type_str),
                ))
            }
        };

        let body = decompressed[null_pos + 1..].to_vec();
        Ok((obj_type, body))
    }

    /// Resolve a hash query (full 64-char or prefix) to a full hash
    pub fn resolve_hash(repo_root: &Path, query: &str) -> HitResult<String> {
        let query = query.trim();
        if query.len() == 64 {
            let prefix = &query[0..2];
            let suffix = &query[2..];
            if Self::objects_dir(repo_root)
                .join(prefix)
                .join(suffix)
                .exists()
            {
                return Ok(query.to_string());
            }
        }

        if query.len() < 4 {
            return Err(HitError::with_hint(
                "Ambiguous hash query",
                "Hash query is too short (minimum 4 characters required).",
                "Provide a longer hash prefix.",
            ));
        }

        let prefix = &query[0..2];
        let suffix_query = &query[2..];
        let dir = Self::objects_dir(repo_root).join(prefix);

        if !dir.exists() {
            return Err(HitError::file_not_found(query));
        }

        let mut matches = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if name.starts_with(suffix_query) {
                matches.push(format!("{}{}", prefix, name));
            }
        }

        match matches.len() {
            0 => Err(HitError::with_hint(
                format!("Object not found: '{}'", query),
                "No matching object in repository.",
                "Check 'hit log' for valid commit hashes.",
            )),
            1 => Ok(matches.into_iter().next().unwrap()),
            _ => Err(HitError::with_hint(
                format!("Ambiguous object reference: '{}'", query),
                format!(
                    "Multiple objects match prefix '{}':\n  {}",
                    query,
                    matches.join("\n  ")
                ),
                "Please provide more characters of the hash to disambiguate.",
            )),
        }
    }
}
