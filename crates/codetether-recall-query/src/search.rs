//! Dispatch query scope after preparing reusable lexical and vector terms.
use crate::SearchOptions;
use anyhow::Result;
use codetether_recall_index::{Query, RecallHit};
use std::{path::Path, time::Instant};

/// Query local recall evidence within a session or across its workspace.
/// # Arguments
/// * `workspace` - Workspace whose catalog is searched when no session is selected.
/// * `query` - Text used for lexical and vector matching.
/// * `options` - Scope, exclusions and score limits.
/// * `host` - Native loading, backfill and persistence capabilities.
/// # Returns
/// Descending-score evidence bounded by the requested limit; empty for a query
/// with no searchable terms.
/// # Errors
/// This boundary preserves a result-based API. Missing or unreadable sources
/// currently yield no evidence, and migration write failures are logged.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_recall_query::Host) -> anyhow::Result<()> {
/// use codetether_recall_query::{run, SearchOptions};
/// let hits = run(std::path::Path::new("."), "deployment policy", SearchOptions {
///     session_id: None, excluded_session: None, limit: 3, minimum_score: 0.2,
/// }, host).await?;
/// assert!(hits.len() <= 3);
/// # Ok(())
/// # }
/// ```
pub async fn run<H: crate::Host>(
    workspace: &Path,
    query: &str,
    options: SearchOptions<'_>,
    host: &H,
) -> Result<Vec<RecallHit>> {
    let started = Instant::now();
    let Some(query) = Query::new(query, options.minimum_score) else {
        return Ok(Vec::new());
    };
    let (hits, sessions) = match options.session_id {
        Some(id) => {
            super::direct::search(id, &query, options.excluded_session, options.limit, host).await
        }
        None => {
            super::workspace::run(
                workspace,
                &query,
                options.excluded_session,
                options.limit,
                host,
            )
            .await
        }
    };
    tracing::debug!(target: "codetether_agent::session::recall::search",
        query_ms = started.elapsed().as_millis(),
        sessions,
        hits = hits.len(),
        "local session recall completed"
    );
    Ok(hits)
}
