//! User configuration location; never defaults to the project workspace.

use anyhow::{Context, Result};
use std::path::PathBuf;

pub(super) fn file() -> Result<PathBuf> {
    let directory = std::env::var_os("CODETETHER_VAULT_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("ai", "codetether", "codetether-agent")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
        .context("Cannot determine the user Vault configuration directory")?;
    anyhow::ensure!(
        directory.is_absolute(),
        "Vault configuration directory must be absolute"
    );
    Ok(directory.join("vault").join("login.bin"))
}
