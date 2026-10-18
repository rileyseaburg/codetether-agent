//! Native journal binding for shared cancellation and durable-goal gates.
/// Check cancellation and durable goal holds before continuing session work.
/// # Errors
/// Propagates cancellation, journal and goal-state errors.
pub async fn ensure_running(session_id: &str) -> anyhow::Result<()> {
    codetether_goal_runtime::ensure_running::<
        codetether_native_goals::runtime::journal::NativeJournal,
    >(session_id)
    .await
}
