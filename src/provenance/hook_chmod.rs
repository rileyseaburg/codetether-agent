use anyhow::Result;
use std::path::Path;

pub fn make_executable(hook_path: &Path) -> Result<()> {
    platform_make_executable(hook_path)
}

#[cfg(unix)]
fn platform_make_executable(hook_path: &Path) -> Result<()> {
    unix::make_executable(hook_path)
}

#[cfg(not(unix))]
fn platform_make_executable(_hook_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(unix)]
mod unix {
    use anyhow::{Context, Result};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    pub fn make_executable(hook_path: &Path) -> Result<()> {
        let mut permissions = fs::metadata(hook_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(hook_path, permissions).context("Failed to chmod commit-msg hook")
    }
}
