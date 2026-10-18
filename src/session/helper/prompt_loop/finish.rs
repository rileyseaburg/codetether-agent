//! Native entry to shared prompt finalization.
#[path = "finish_host.rs"]
mod host;
use super::Runner;
use crate::session::SessionResult;
pub(super) async fn finish(runner: &mut Runner<'_>) -> anyhow::Result<SessionResult> {
    codetether_prompt_loop::lifecycle::finish::run(runner).await
}
