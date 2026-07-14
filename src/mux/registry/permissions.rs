//! Owner-only filesystem permissions for mux credentials.

use anyhow::Result;

#[cfg(unix)]
pub(super) fn owner_only_dir(path: &std::path::Path) -> Result<()> {
    chmod(path, 0o700)
}
#[cfg(unix)]
pub(super) fn owner_only_file(path: &std::path::Path) -> Result<()> {
    chmod(path, 0o600)
}
#[cfg(not(unix))]
pub(super) fn owner_only_dir(_: &std::path::Path) -> Result<()> {
    Ok(())
}
#[cfg(not(unix))]
pub(super) fn owner_only_file(_: &std::path::Path) -> Result<()> {
    Ok(())
}

#[cfg(unix)]
fn chmod(path: &std::path::Path, mode: u32) -> Result<()> {
    use anyhow::Context;
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .context("secure mux registry permissions")
}
