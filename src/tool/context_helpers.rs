//! Shared session loader for context tools.
//!
//! Distinguishes "no sessions exist" from real I/O/parse errors.

use crate::session::Session;
use anyhow::Result;

/// Load the latest session for the current working directory.
///
/// Returns `Ok(None)` when no sessions exist, `Ok(Some(session))`
/// on success, or `Err` on I/O/parse failures.
pub async fn load_latest_session() -> Result<Option<Session>> {
    let cwd = std::env::current_dir().ok();
    match Session::last_for_directory(cwd.as_deref()).await {
        Ok(s) => Ok(Some(s)),
        Err(e) => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("no session")
                || msg.contains("not found")
                || msg.contains("no such file")
            {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}
