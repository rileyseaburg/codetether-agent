//! Delegate the full sequential story lifecycle to its dedicated crate.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) async fn run_sequential(&mut self) -> anyhow::Result<()> {
        codetether_ralph_sequential::run(self).await
    }
}
