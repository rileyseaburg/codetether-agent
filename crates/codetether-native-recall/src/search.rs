//! Local indexed evidence queries.
pub use codetether_recall_query::SearchOptions;
/// Search native recall evidence in the requested workspace.
/// # Arguments
/// * `workspace` - Workspace used to scope indexed evidence.
/// * `query` - Search text.
/// * `options` - Session filters and result limits.
/// # Returns
/// Ranked recall hits retaining their original provenance.
/// # Errors
/// Propagates shared recall-query errors.
/// # Examples
/// `run::<S>(workspace, query, options).await?` uses the native recall host.
pub async fn run<S: crate::Session>(
    workspace: &std::path::Path,
    query: &str,
    options: SearchOptions<'_>,
) -> anyhow::Result<Vec<codetether_session_recall::RecallHit>> {
    codetether_recall_query::run(workspace, query, options, &crate::Host::<S>::default()).await
}
