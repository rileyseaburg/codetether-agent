//! Native entry to shared step preparation.
#[path = "begin_host/mod.rs"]
mod host;
use super::super::Runner;
pub(super) async fn run(runner: &mut Runner<'_>, step: usize) -> anyhow::Result<()> {
    codetether_prompt_loop::lifecycle::start::run(runner, step).await
}
