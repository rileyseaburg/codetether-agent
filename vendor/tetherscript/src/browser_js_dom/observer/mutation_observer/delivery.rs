//! Mutation observer delivery.

use super::registry::MutationObserverRegistry;

impl MutationObserverRegistry {
    pub fn deliver(&mut self) {
        for obs in &mut self.observers {
            let records = std::mem::take(&mut obs.pending);
            if !records.is_empty() {
                (obs.callback)(records);
            }
        }
    }
}
