//! Stable socket paths derived only from durable session identity.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use directories::ProjectDirs;
use sha2::{Digest, Sha256};

pub(super) fn for_session(session_id: &str) -> Result<PathBuf> {
    let root = if let Some(path) = std::env::var_os("CODETETHER_SESSION_RUNTIME_DIR") {
        PathBuf::from(path)
    } else {
        ProjectDirs::from("ai", "codetether", "codetether-agent")
            .context("CodeTether runtime directory is unavailable")?
            .data_dir()
            .join("session-runtime")
    };
    for_session_in(&root, session_id)
}

pub(super) fn for_session_in(root: &Path, session_id: &str) -> Result<PathBuf> {
    ensure!(
        (8..=128).contains(&session_id.len())
            && session_id
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_')),
        "invalid session id"
    );
    std::fs::create_dir_all(root)?;
    #[cfg(unix)]
    std::fs::set_permissions(root, std::os::unix::fs::PermissionsExt::from_mode(0o700))?;
    let digest = hex::encode(Sha256::digest(session_id.as_bytes()));
    Ok(root.join(format!("{}.sock", &digest[..32])))
}
