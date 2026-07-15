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
    let Some(query) = super::rank::Query::new(query, options.minimum_score) else {
        return Ok(Vec::new());
    };
    let (hits, sessions) = match options.session_id {
        Some(id) => direct(id, &query, options.excluded_session, options.limit).await,
        None => {
            super::search_workspace::run(workspace, &query, options.excluded_session, options.limit)
                .await
        }
    };
    tracing::debug!(
        query_ms = started.elapsed().as_millis(),
        sessions,
        hits = hits.len(),
        "local session recall completed"
    );
    Ok(hits)
}

async fn direct(
    id: &str,
    query: &super::rank::Query,
    excluded: Option<&str>,
    limit: usize,
) -> (Vec<RecallHit>, usize) {
    let Some(session) = super::load::session(id).await else {
        return (Vec::new(), 0);
    };
    let mut hits = Vec::new();
    super::rank::merge(&mut hits, query.hits(&session, excluded), limit);
    (hits, 1)
}
