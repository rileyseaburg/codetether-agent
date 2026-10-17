//! Top-level Ralph run lifecycle and scheduling-mode selection.
use super::*;

impl RalphLoop {
    /// Run the Ralph loop until completion or max iterations
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        self.start_run()?;
        // Choose execution mode
        if self.config.parallel_enabled {
            self.run_parallel().await?;
        } else {
            self.run_sequential().await?;
        }

        codetether_ralph_core::lifecycle::finalize_status(&mut self.state);
        self.cleanup_run_worktrees().await;
        self.finish_run()
    }
}
