//! Shared user authentication path operations.

use anyhow::Result;

/// Get the path to the credential storage file.
pub(super) fn credential_file_path() -> Result<std::path::PathBuf> {
    use directories::ProjectDirs;
    let dirs = ProjectDirs::from("ai", "codetether", "codetether-agent")
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?;
    Ok(dirs.config_dir().join("credentials.json"))
}
