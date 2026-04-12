//! Credential-helper script generation.
//!
//! Repositories use a tiny shell wrapper that re-enters the current CodeTether
//! binary with the right `git-credential-helper` arguments.
//!
//! # Examples
//!
//! ```ignore
//! write_git_credential_helper_script(script_path, "ws-1")?;
//! ```

use anyhow::{Context, Result};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Writes the repository-local shell wrapper used by Git credential helpers.
///
/// The generated script is marked executable on Unix platforms.
///
/// # Examples
///
/// ```ignore
/// write_git_credential_helper_script(script_path, "ws-1")?;
/// ```
pub fn write_git_credential_helper_script(script_path: &Path, workspace_id: &str) -> Result<()> {
    if let Some(parent) = script_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create helper script directory {}",
                parent.display()
            )
        })?;
    }
    let current_exe = std::env::current_exe()
        .context("Failed to determine current CodeTether binary path for credential helper")?;
    let script = format!(
        "#!/bin/sh\nexec {} git-credential-helper --workspace-id {} \"$@\"\n",
        shell_single_quote(current_exe.to_string_lossy().as_ref()),
        shell_single_quote(workspace_id)
    );
    fs::write(script_path, script).with_context(|| {
        format!(
            "Failed to write Git credential helper script {}",
            script_path.display()
        )
    })?;
    #[cfg(unix)]
    mark_executable(script_path)?;
    Ok(())
}

#[cfg(unix)]
fn mark_executable(script_path: &Path) -> Result<()> {
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

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
