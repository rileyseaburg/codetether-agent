//! Screenshot file writing for browserctl.

use crate::browser::output::ScreenshotData;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Write screenshot bytes to disk and return the output path.
///
/// # Errors
///
/// Returns an error when no output path is supplied or the file cannot be
/// written.
pub(super) async fn write(
    input: &super::input::BrowserCtlInput,
    screenshot: ScreenshotData,
    metadata: &mut HashMap<String, Value>,
) -> Result<Value> {
    let path = resolve_path(
        input
            .path
            .as_deref()
            .context("path is required for screenshot")?,
    );
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(&path, screenshot.bytes).await?;
    metadata.insert("path".into(), json!(path.display().to_string()));
    metadata.insert("file".into(), json!({ "path": path.display().to_string(), "exists": true, "absolute": path.is_absolute() }));
    Ok(json!({ "path": path.display().to_string() }))
}

fn resolve_path(raw: &str) -> PathBuf {
    if let Some(rest) = raw.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return Path::new(&home).join(rest);
    }
    PathBuf::from(raw)
}
