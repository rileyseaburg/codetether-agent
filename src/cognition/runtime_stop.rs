//! Native facade for shared cognition stop behavior.
use super::{CognitionRuntime, CognitionStatus};
use anyhow::Result;
impl CognitionRuntime {
    /// Stop the loop, abort its task, and record an optional reason.
    ///
    /// # Errors
    ///
    /// Returns an error only if status collection fails.
    pub async fn stop(&self, reason: Option<String>) -> Result<CognitionStatus> {
        self.state.stop(reason).await
    }
}
