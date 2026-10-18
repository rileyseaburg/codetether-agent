//! Native session specialization of explicit auto-goal adoption.
/// Adopt an explicit goal prompt when automatic goal adoption is enabled.
pub async fn adopt_prompt(session_id: &str, prompt: &str) -> anyhow::Result<bool> {
    codetether_goal_runtime::adopt_prompt::<super::journal::NativeJournal>(session_id, prompt).await
}
