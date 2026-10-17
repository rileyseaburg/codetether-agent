//! Test adapter for dispatch against application task logs.
pub(super) async fn run(
    log: &crate::session::tasks::TaskLog,
    params: serde_json::Value,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_session_task::dispatch::<super::host::ApplicationHost>(log, params).await
}
