//! Test adapter for dispatch against application task logs.
pub(super) async fn run(
    log: &codetether_native_goals::TaskLog,
    params: serde_json::Value,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    codetether_session_task::dispatch::<crate::tests::TestHost>(log, params).await
}
