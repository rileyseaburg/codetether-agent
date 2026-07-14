//! Public local-query boundary for session recall consumers.

use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use super::hit::RecallHit;

/// Fully typed controls for one local recall query.
pub(crate) struct SearchOptions<'a> {
    pub session_id: Option<&'a str>,
    pub excluded_session: Option<&'a str>,
    pub limit: usize,
    pub minimum_score: f32,
}

pub(crate) async fn run(
    workspace: &Path,
    query: &str,
    options: SearchOptions<'_>,
) -> Result<Vec<RecallHit>> {
    let started = Instant::now();
    let source = super::search_source::SearchSource::load(workspace, options.session_id).await;
    let sessions = source.sessions();
    let hits = super::rank::search(
        &sessions,
        query,
        options.excluded_session,
        options.limit,
        options.minimum_score,
    );
    tracing::debug!(
        query_ms = started.elapsed().as_millis(),
        sessions = sessions.len(),
        hits = hits.len(),
        "local session recall completed"
    );
    Ok(hits)
}
