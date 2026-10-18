//! Summary status reporting and internal event publishing.

use super::{CognitionRuntime, CognitionStatus, ThoughtEvent};
use crate::Host;

impl<H: Host> CognitionRuntime<H> {
    /// Return a summary of runtime and persona state.
    pub async fn status(&self) -> CognitionStatus {
        self.state.status().await
    }

    /// Append an event to the bounded buffer and broadcast it.
    pub(super) async fn push_event(&self, event: ThoughtEvent) {
        self.state.push_event(event).await
    }
}
