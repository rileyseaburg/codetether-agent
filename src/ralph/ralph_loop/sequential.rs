//! Sequential scheduling loop with focused per-iteration handlers.
use super::*;
impl RalphLoop {
    pub(super) async fn run_sequential(&mut self) -> anyhow::Result<()> {
        codetether_ralph_core::scheduling::run_sequential(self).await
    }
}
