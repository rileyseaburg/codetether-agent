//! Native session specialization of shared goal usage accounting.
pub(crate) async fn record_usage(
    session_id: &str,
    tokens: usize,
    elapsed: std::time::Duration,
) -> anyhow::Result<()> {
    codetether_goal_runtime::record_usage::<super::journal::NativeJournal>(
        session_id, tokens, elapsed,
    )
    .await
}
