//! Credential configuration operation: mark executable.
use super::*;

#[cfg(unix)]
pub(super) fn mark_executable(script_path: &Path) -> Result<()> {
    let mut permissions = fs::metadata(script_path)
        .with_context(|| {
            format!(
                "Failed to stat Git credential helper script {}",
                script_path.display()
            )
        })?
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(script_path, permissions).with_context(|| {
        format!(
            "Failed to mark Git credential helper script executable {}",
            script_path.display()
        )
    })
}
