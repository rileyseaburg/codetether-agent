//! Load and sort the combined catalog with native failures propagated.
use anyhow::Result;
use codetether_session_listing::list_sessions_for_directory;
use codetether_session_metadata::SessionSummary;
use std::{collections::HashMap, path::Path};

/// Merge native and Codex summaries, sorted by latest update.
/// # Arguments
/// * `dir` — Workspace directory whose sessions should be listed.
/// # Returns
/// Matching summaries sorted by descending update time.
/// # Errors
/// Propagates native listing failures; Codex failures are logged and skipped.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_listing::Source>() -> anyhow::Result<()> {
/// let rows = codetether_session_catalog::list_all_sessions_for_directory::<S>(std::path::Path::new(".")).await?;
/// assert!(rows.windows(2).all(|w| w[0].updated_at >= w[1].updated_at));
/// # Ok(()) }
/// ```
pub async fn list_all_sessions_for_directory<S: codetether_session_listing::Source>(
    dir: &Path,
) -> Result<Vec<SessionSummary>> {
    tracing::info!(target: "codetether_agent::session::listing_all", dir = %dir.display(), "list_all_sessions_for_directory: starting");
    let mut merged = HashMap::<String, SessionSummary>::new();

    let native = list_sessions_for_directory::<S>(dir).await?;
    tracing::info!(target: "codetether_agent::session::listing_all",
        native_count = native.len(),
        "list_all_sessions_for_directory: native sessions"
    );

    for session in native {
        merged.insert(session.id.clone(), session);
    }

    let codex = super::imported::discover(dir).await;
    super::merge::merge(&mut merged, codex);

    let mut sessions = merged.into_values().collect::<Vec<_>>();
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    tracing::info!(target: "codetether_agent::session::listing_all",
        total = sessions.len(),
        "list_all_sessions_for_directory: done"
    );
    Ok(sessions)
}
