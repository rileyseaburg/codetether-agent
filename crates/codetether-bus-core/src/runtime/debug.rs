//! Standard bus trait implementation.
use super::*;

impl std::fmt::Debug for AgentBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentBus")
            .field("subscribers", &self.tx.receiver_count())
            .finish()
    }
}
