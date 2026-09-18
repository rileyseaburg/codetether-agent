//! Atomic profile persistence; a failed login never reaches this layer.

use super::{Profile, crypto, paths};
use anyhow::{Result, ensure};

pub(crate) fn load() -> Result<Option<Profile>> {
    let file = paths::file()?;
    let metadata = match std::fs::symlink_metadata(&file) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => anyhow::bail!("Cannot read the saved Vault profile"),
    };
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "Vault profile must be a regular file"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        ensure!(
            metadata.permissions().mode() & 0o077 == 0,
            "Vault profile permissions must be owner-only"
        );
    }
    ensure!(
        metadata.len() <= 65536,
        "Vault profile exceeds its size limit"
    );
    let bytes = crypto::decode(&std::fs::read(file)?)?;
    let mut profile: Profile = serde_json::from_slice(&bytes).map_err(|_| {
        anyhow::anyhow!("Saved Vault profile is invalid; existing data was preserved")
    })?;
    profile.address = super::normalize(&profile.address)?;
    Ok(Some(profile))
}

pub(crate) use super::save::save;
