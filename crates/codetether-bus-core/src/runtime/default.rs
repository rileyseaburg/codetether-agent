//! Standard bus trait implementation.
use super::*;

impl Default for AgentBus {
    fn default() -> Self {
        Self::new()
    }
}
