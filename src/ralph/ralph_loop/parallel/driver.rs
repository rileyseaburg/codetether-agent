//! Delegate dependency-stage coordination to its dedicated crate.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn run_parallel(&mut self) -> anyhow::Result<()> {
        codetether_ralph_stage::run(self).await
    }
}
