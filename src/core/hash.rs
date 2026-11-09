use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Compute BLAKE3 hex hash of in-memory data
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    hasher.finalize().to_hex().to_string()
}

/// Compute BLAKE3 hex hash directly from file using a buffered reader
#[allow(dead_code)]
pub fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 64 * 1024]; // 64KB buffer for optimal I/O throughput
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// Return short 7-character hash prefix
pub fn short_hash(hash: &str) -> &str {
    if hash.len() >= 7 {
        &hash[..7]
    } else {
        hash
    }
}
