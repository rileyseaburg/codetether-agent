//! Delegate the full sequential story lifecycle to its dedicated crate.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn run_sequential(&mut self) -> anyhow::Result<()> {
        codetether_ralph_sequential::run(self).await
    }
}
