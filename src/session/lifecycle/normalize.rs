//! Native bus attachment followed by shared loaded-session hydration.
use crate::session::types::Session;
impl Session {
    /// Rebuild runtime state omitted from persisted or legacy sessions.
    pub(crate) fn normalize_sidecars(&mut self) {
        self.attach_global_bus_if_missing();
        codetether_session_metadata::lifecycle::hydrate(
            &mut self.metadata,
            &self.messages,
            &mut self.pages,
        );
    }
}
