//! Avoid rewriting unchanged turn text and preserve file I/O error context.
use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

pub(super) async fn write_turn_if_changed(path: &Path, body: &str) -> Result<()> {
    match fs::read_to_string(path).await {
        Ok(existing) if existing == body => return Ok(()),
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err).with_context(|| format!("read {}", path.display())),
    }
    fs::write(path, body)
        .await
        .with_context(|| format!("write {}", path.display()))
}
