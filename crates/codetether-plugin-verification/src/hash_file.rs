//! hash file for verified plugin registration.
use super::*;

/// Compute SHA-256 hash of file contents.
pub fn hash_file(path: &Path) -> Result<String> {
    let contents = std::fs::read(path)
        .with_context(|| format!("Failed to read file for hashing: {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(hex::encode(hasher.finalize()))
}
