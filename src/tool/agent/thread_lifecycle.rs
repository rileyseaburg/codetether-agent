//! Application bindings for shared durable-child lifecycle orchestration.
#[path = "thread_lifecycle/host.rs"]
mod host;
pub(super) async fn close(
    target: &str,
    owner: Option<&str>,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_agent_lifecycle::close::<host::ApplicationHost>(target, owner).await
}
pub(super) async fn resume(
    target: &str,
    owner: Option<&str>,
    config: super::residency::ResumeConfig,
) -> anyhow::Result<crate::tool::ToolResult> {
    codetether_agent_lifecycle::resume::<host::ApplicationHost>(target, owner, config).await
}

#[cfg(test)]
#[path = "thread_lifecycle/subtree_close_tests.rs"]
mod subtree_close_tests;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_resume_tests.rs"]
mod subtree_resume_tests;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_test_support.rs"]
mod subtree_test_support;
#[cfg(test)]
#[path = "thread_lifecycle/subtree_transition_tests.rs"]
mod subtree_transition_tests;
