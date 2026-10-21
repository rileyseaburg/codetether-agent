//! Private cache directory creation, without following a final symlink.
use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

pub(super) fn default_path() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("CODETETHER_DIAGNOSTICS_DIR") {
        ensure!(
            !path.is_empty(),
            "CODETETHER_DIAGNOSTICS_DIR cannot be empty"
        );
        return Ok(PathBuf::from(path));
    }
    Ok(dirs::cache_dir()
        .context("user cache directory unavailable")?
        .join("codetether")
        .join("diagnostics"))
}

pub(super) fn prepare(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)?;
    }
    #[cfg(not(unix))]
    std::fs::create_dir_all(path)?;
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "diagnostics directory must not be a symlink"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        // SAFETY: geteuid has no arguments and only returns the calling identity.
        ensure!(
            metadata.uid() == unsafe { libc::geteuid() },
            "diagnostics directory belongs to another user"
        );
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

pub(super) fn private_file(file: &std::fs::File) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    #[cfg(not(unix))]
    let _ = file;
    Ok(())
}
