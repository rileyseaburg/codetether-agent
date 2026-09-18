//! Atomic replacement of the private Vault profile.
use super::{Profile, crypto, paths};
use anyhow::{Context, Result};
use std::io::Write;
pub(crate) fn save(profile: &Profile) -> Result<()> {
    let file = paths::file()?;
    let parent = file.parent().context("Missing Vault profile directory")?;
    std::fs::create_dir_all(parent)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700))?;
    }
    let data = serde_json::to_vec(profile)?;
    anyhow::ensure!(data.len() <= 65536, "Vault profile exceeds its size limit");
    let bytes = crypto::encode(&data)?;
    let mut pending = tempfile::NamedTempFile::new_in(parent)?;
    pending.write_all(&bytes)?;
    pending.as_file().sync_all()?;
    pending.persist(file).map_err(|_| {
        anyhow::anyhow!("Could not publish the Vault profile; previous settings were retained")
    })?;
    Ok(())
}
