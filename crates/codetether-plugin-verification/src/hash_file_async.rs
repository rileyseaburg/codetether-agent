//! hash file async for verified plugin registration.
use super::*;

pub(super) async fn hash_file_async(path: &Path) -> Result<String> {
    let contents = tokio::fs::read(path)
        .await
        .with_context(|| format!("Failed to read file for hashing: {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(hex::encode(hasher.finalize()))
}
