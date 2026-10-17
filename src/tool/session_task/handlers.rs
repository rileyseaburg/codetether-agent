//! Application integration coverage for shared goal-action guards.
#[path = "handlers/clear_guard_tests.rs"]
mod clear_guard_tests;
#[path = "handlers/set_guard_tests.rs"]
mod set_guard_tests;
pub(super) async fn list(
    log: &crate::session::tasks::TaskLog,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_session_task::list_tasks(log).await
}
