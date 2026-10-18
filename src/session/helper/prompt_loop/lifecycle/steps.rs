//! Native entry to the shared provider/tool lifecycle.
#[path = "host.rs"]
mod host;
use super::super::Runner;
use crate::session::SessionResult;
pub(super) async fn run(runner: &mut Runner<'_>) -> anyhow::Result<SessionResult> {
    codetether_prompt_loop::lifecycle::run(runner).await
}
