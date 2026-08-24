//! Workspace-confined loading of local image bytes.

use super::ImageTool;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) async fn load(path: &str, args: &Value) -> Result<(String, String, usize, String)> {
    let root = crate::tool::network_access::trusted_workspace(args)
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?)
        .canonicalize()?;
    let requested = Path::new(path);
    let requested = if requested.is_absolute() { requested.into() } else { root.join(requested) };
    let resolved = requested
        .canonicalize()
        .map_err(|error| anyhow!("Image file not found: {path}: {error}"))?;
    if !resolved.starts_with(&root) {
        return Err(anyhow!("image path is outside workspace: {}", resolved.display()));
    }
    let data = tokio::fs::read(&resolved).await?;
    let mime = ImageTool::detect_mime_type(path).to_string();
    let encoded = ImageTool::encode_as_data_url(&data, &mime);
    Ok((encoded, mime, data.len(), resolved.display().to_string()))
}