//! Native binding for shared goal failure recovery.
#[path = "goal_recovery/host.rs"]
mod host;
use super::Runner;
pub(super) async fn after(runner: &mut Runner<'_>, error: &anyhow::Error) -> bool {
    codetether_prompt_loop::goal_recovery::after(runner, error).await
}
