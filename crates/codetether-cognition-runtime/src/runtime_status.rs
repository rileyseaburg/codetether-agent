//! Summary status reporting and internal event publishing.

use super::{CognitionRuntime, CognitionStatus};
use crate::Host;

impl<H: Host> CognitionRuntime<H> {
    /// Return a summary of runtime and persona state.
    pub async fn status(&self) -> CognitionStatus {
        self.state.status().await
    }
}
