//! Delegate dependency-stage coordination to its dedicated crate.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) async fn run_parallel(&mut self) -> anyhow::Result<()> {
        codetether_ralph_stage::run(self).await
    }
}
