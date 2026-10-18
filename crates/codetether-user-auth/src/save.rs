//! Shared user authentication save operations.

use super::path::credential_file_path;
use crate::LoginResponsePayload;
use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;

/// Persist the selected server and its returned user credential.
///
/// # Arguments
/// * `server_url` - Server base URL stored in the record.
/// * `email` - Account email stored in the record.
/// * `login` - Successful server login response.
/// # Returns
/// The platform configuration path of the written JSON record.
/// # Errors
/// Returns configuration-directory, file creation or serialization errors.
/// # Examples
/// ```no_run
/// # fn example(reply: &codetether_user_auth::LoginResponsePayload) -> anyhow::Result<()> {
/// let path = codetether_user_auth::write_saved_credentials("https://example.invalid", "user@example.invalid", reply)?;
/// # Ok(()) }
/// ```
pub fn write_saved_credentials(
    server_url: &str,
    email: &str,
    login: &LoginResponsePayload,
) -> Result<PathBuf> {
    // Store token to ~/.config/codetether-agent/credentials.json
    let cred_path = credential_file_path()?;
    if let Some(parent) = cred_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config dir: {}", parent.display()))?;
    }

    let creds = json!({
        "server": server_url,
        "access_token": login.access_token,
        "expires_at": login.expires_at,
        "email": email,
    });

    // Write with restrictive permissions (owner-only read/write)
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&cred_path)
            .with_context(|| format!("Failed to write credentials to {}", cred_path.display()))?;
        serde_json::to_writer_pretty(file, &creds)?;
    }
    #[cfg(not(unix))]
    {
        let file = std::fs::File::create(&cred_path)
            .with_context(|| format!("Failed to write credentials to {}", cred_path.display()))?;
        serde_json::to_writer_pretty(file, &creds)?;
    }

    Ok(cred_path)
}
