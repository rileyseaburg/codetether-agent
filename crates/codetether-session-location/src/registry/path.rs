//! Registry root selection and entry filenames.
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub(super) fn entry(id: &str) -> Result<PathBuf> {
    let root = if let Some(path) = explicit_root() {
        path
    } else {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .context("resolve global CodeTether data directory")?
            .data_dir()
            .to_path_buf()
    };
    Ok(root.join("session-locations").join(format!("{id}.path")))
}

fn explicit_root() -> Option<PathBuf> {
    std::env::var("CODETETHER_DATA_DIR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}
