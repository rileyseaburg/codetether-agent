//! Discover Codex sessions off the async executor, preserving soft failures.
use super::codex::list_codex_sessions_for_directory;
use codetether_session_metadata::SessionSummary;
use std::path::Path;
pub(crate) async fn discover(dir: &Path) -> Vec<SessionSummary> {
    // Codex discovery walks and reads every archive file synchronously; run it
    // off the async executor so a TUI event loop awaiting this stays responsive.
    let dir_owned = dir.to_path_buf();
    let codex = tokio::task::spawn_blocking(move || list_codex_sessions_for_directory(&dir_owned))
        .await
        .map_err(anyhow::Error::from)
        .and_then(|result| result);
    match codex {
        Ok(c) => {
            tracing::info!(target: "codetether_agent::session::listing_all",
                codex_count = c.len(),
                "list_all_sessions_for_directory: codex sessions"
            );
            c
        }
        Err(err) => {
            tracing::warn!(target: "codetether_agent::session::listing_all", error = %err, "list_all_sessions_for_directory: codex discovery failed");
            Vec::new()
        }
    }
}
