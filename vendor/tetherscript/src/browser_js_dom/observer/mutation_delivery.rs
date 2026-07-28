//! Deliver pending mutation records.

use super::mutation_observer::MutationObserverRegistry;

/// Flush all pending mutation records to their callbacks.
pub fn deliver_pending_mutations(registry: &mut MutationObserverRegistry) {
    registry.deliver();
}
