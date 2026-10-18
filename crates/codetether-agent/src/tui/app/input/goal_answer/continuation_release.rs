//! Native journal binding for shared continuation release tests.
async fn release(id: &str) -> anyhow::Result<bool> {
    codetether_goal_answer::release::release::<super::super::review_host::Native>(id).await
}
#[path = "continuation_release_tests.rs"]
mod tests;
