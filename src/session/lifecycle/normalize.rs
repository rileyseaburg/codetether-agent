//! Runtime-sidecar hydration for loaded sessions.

use crate::session::{pages::classify_all, types::Session};

impl Session {
    /// Rebuild runtime state omitted from persisted or legacy sessions.
    pub(crate) fn normalize_sidecars(&mut self) {
        self.attach_global_bus_if_missing();
        if self.pages.len() != self.messages.len() {
            self.pages = classify_all(&self.messages);
        }
        if self.metadata.history_sink.is_none() {
            self.metadata.history_sink =
                crate::session::history_sink::HistorySinkConfig::from_env()
                    .ok()
                    .flatten();
        }
        if let Some(identity) = crate::provenance::runtime_agent_identity()
            && let Some(provenance) = self.metadata.provenance.as_mut()
        {
            provenance.identity.agent_identity_id = Some(identity);
        }
    }
}
