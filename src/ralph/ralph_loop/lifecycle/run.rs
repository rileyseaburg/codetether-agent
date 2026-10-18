//! Delegate the complete run lifecycle to its dedicated crate.
use super::*;
impl RalphLoop {
    /// Run the Ralph loop until completion or max iterations.
    pub async fn run(&mut self) -> anyhow::Result<RalphState> {
        codetether_ralph_run::run(self).await
    }
}
