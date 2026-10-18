//! Native recall queries specialized to the application session.
pub(crate) use codetether_native_recall::search::SearchOptions;
pub(crate) async fn run(
    workspace: &std::path::Path,
    query: &str,
    options: SearchOptions<'_>,
) -> anyhow::Result<Vec<super::hit::RecallHit>> {
    codetether_native_recall::search::run::<crate::session::Session>(workspace, query, options)
        .await
}
