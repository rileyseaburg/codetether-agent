//! Application integration coverage for shared goal-action guards.
#[path = "handlers/clear_guard_tests.rs"]
mod clear_guard_tests;
#[path = "handlers/set_guard_tests.rs"]
mod set_guard_tests;
pub(super) async fn list(
    log: &codetether_native_goals::TaskLog,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    codetether_session_task::list_tasks(log).await
}
