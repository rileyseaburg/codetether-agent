//! Native journal binding for shared cancellation and durable-goal gates.
pub(crate) async fn ensure_running(session_id: &str) -> anyhow::Result<()> {
    codetether_goal_runtime::ensure_running::<super::goals::journal::NativeJournal>(session_id)
        .await
}
