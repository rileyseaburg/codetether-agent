//! Bind checkout and the two scheduling modes to their existing entry points.
use super::*;
#[async_trait::async_trait]
impl codetether_ralph_run::Execution for RalphLoop {
    fn git_checkout(&self, branch: &str) -> anyhow::Result<()> {
        self.git_checkout(branch)
    }
    async fn run_parallel(&mut self) -> anyhow::Result<()> {
        self.run_parallel().await
    }
    async fn run_sequential(&mut self) -> anyhow::Result<()> {
        self.run_sequential().await
    }
}
