//! Coordinate dependency stages, isolated task spawning and result collection.
use super::*;
impl RalphLoop {
    /// Run stories in parallel by stage with worktree isolation
    pub(in crate::ralph::ralph_loop) async fn run_parallel(&mut self) -> anyhow::Result<()> {
        codetether_ralph_core::scheduling::run_parallel(self).await
    }
}
