//! state for relay coordination.
use super::*;

#[derive(Clone)]
pub(super) struct RelayRuntimeState {
    pub(super) runtime: Arc<ProtocolRelayRuntime>,
    pub(super) participants: Vec<String>,
}

impl RelayRuntimeState {
    pub(super) fn new(runtime: Arc<ProtocolRelayRuntime>) -> Self {
        Self {
            runtime,
            participants: Vec::new(),
        }
    }

    pub(super) fn remember_participant(&mut self, participant: &str) {
        if !self.participants.iter().any(|name| name == participant) {
            self.participants.push(participant.to_string());
        }
    }
}
