//! Workspace/language backoff prevents every patch retrying an unavailable server.

use std::path::Path;
use tokio::time::{Duration, Instant};
#[path = "cooldown_store.rs"]
mod store;

const BACKOFF: Duration = Duration::from_secs(30);

pub(super) fn reason(workspace: &Path, language: &str) -> Option<String> {
    store::entries()
        .get(&(workspace.to_path_buf(), language.to_string()))
        .map(|(_, message)| message.clone())
}

pub(super) fn record(workspace: &Path, language: &str, reason: &str) {
    store::entries().insert(
        (workspace.to_path_buf(), language.to_string()),
        (
            Instant::now() + BACKOFF,
            format!("LSP preflight temporarily backed off after: {reason}"),
        ),
    );
}

pub(super) fn begin(workspace: &Path, language: &str) -> anyhow::Result<()> {
    let mut entries = store::entries();
    let key = (workspace.to_path_buf(), language.to_string());
    if let Some((_, reason)) = entries.get(&key) {
        anyhow::bail!("{reason}");
    }
    entries.insert(
        key,
        (
            Instant::now() + Duration::from_secs(61),
            "LSP diagnostics are still running in the background; the server is retained".into(),
        ),
    );
    Ok(())
}

pub(super) fn clear(workspace: &Path, language: &str) {
    store::entries().remove(&(workspace.to_path_buf(), language.to_string()));
}

#[cfg(test)]
#[path = "cooldown_tests.rs"]
mod tests;
