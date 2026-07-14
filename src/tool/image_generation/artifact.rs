use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::path::PathBuf;

pub(super) async fn save(encoded: &str) -> Result<PathBuf> {
    let bytes = STANDARD
        .decode(encoded.trim())
        .context("Images API returned invalid base64")?;
    let root = crate::config::Config::data_dir()
        .unwrap_or_else(|| PathBuf::from(".codetether-agent"))
        .join("image-generations");
    tokio::fs::create_dir_all(&root)
        .await
        .with_context(|| format!("failed to create `{}`", root.display()))?;
    let path = root.join(format!("{}.png", uuid::Uuid::new_v4()));
    tokio::fs::write(&path, bytes)
        .await
        .with_context(|| format!("failed to save `{}`", path.display()))?;
    Ok(path)
}
