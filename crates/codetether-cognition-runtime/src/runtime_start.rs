//! Native facade for shared cognition start behavior.
use super::{CognitionRuntime, CognitionStatus, StartCognitionRequest};
use crate::Host;
use anyhow::Result;
impl<H: Host> CognitionRuntime<H> {
    /// Start the perpetual cognition loop.
    ///
    /// Seeds a root persona when none exist. Calling this while already running
    /// is a no-op that returns current status.
    ///
    /// # Errors
    ///
    /// Returns an error when cognition is disabled, or when seeding the initial
    /// persona fails.
    pub async fn start(&self, req: Option<StartCognitionRequest>) -> Result<CognitionStatus> {
        self.state.start(req, H::drive).await
    }
}
