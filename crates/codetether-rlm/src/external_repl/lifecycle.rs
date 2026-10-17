//! Explicit child termination and runtime inspection.
use super::*;

impl ExternalRepl {
    /// Kill the REPL process
    ///
    /// # Errors
    /// Returns an error if Tokio cannot terminate or wait for the child.
    /// # Examples
    /// ```no_run
    /// # async fn example(repl: &mut codetether_rlm::external_repl::ExternalRepl) -> anyhow::Result<()> {
    /// repl.destroy().await?;
    /// # Ok(()) }
    /// ```
    pub async fn destroy(&mut self) -> Result<()> {
        tracing::debug!(runtime = ?self.runtime, "Destroying external REPL");
        self.child.kill().await?;
        Ok(())
    }

    /// Get the runtime type used by this REPL
    pub fn runtime(&self) -> ReplRuntime {
        self.runtime
    }
}
