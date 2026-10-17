//! Iterate sequential stories within the live host iteration limit.
use super::*;
/// Run sequential scheduling until the host finishes or exhausts its limit.
///
/// # Arguments
/// `host` supplies live state and story execution callbacks.
/// # Returns
/// Completes when no more stories can run.
/// # Errors
/// Propagates successful-response handling errors; model errors are recorded.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_core::scheduling::SequentialHost) -> anyhow::Result<()> {
/// codetether_ralph_core::scheduling::run_sequential(host).await
/// # }
/// ```
pub async fn run_sequential(host: &mut impl SequentialHost) -> Result<()> {
    while host.state().current_iteration < host.state().max_iterations {
        let Some(story) = host.begin() else {
            break;
        };
        let prompt = host.prompt(&story);
        match host.call(&story.id, &prompt).await {
            Ok(response) => host.response(&story, &response).await?,
            Err(e) => host.error(&story, &e),
        }
    }
    Ok(())
}
