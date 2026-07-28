//! Mutation observer registry operations.

use super::super::mutation_types::{MutationObserverConfig, MutationRecord};
use super::entry::{MutationCallback, MutationObserver};

#[derive(Default)]
pub struct MutationObserverRegistry {
    next_id: u64,
    pub(super) observers: Vec<MutationObserver>,
}

impl MutationObserverRegistry {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            observers: vec![],
        }
    }

    pub fn observe(
        &mut self,
        target: u64,
        config: MutationObserverConfig,
        cb: MutationCallback,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.observers
            .push(MutationObserver::new(id, target, config, cb));
        id
    }

    pub fn disconnect(&mut self, id: u64) {
        self.observers.retain(|o| o.id != id);
    }

    pub fn take_records(&mut self, id: u64) -> Vec<MutationRecord> {
        self.observers
            .iter_mut()
            .find(|o| o.id == id)
            .map(|o| std::mem::take(&mut o.pending))
            .unwrap_or_default()
    }

    pub fn queue_record(&mut self, record: MutationRecord, ancestors: &[u64]) {
        for obs in &mut self.observers {
            if obs.matches(&record, ancestors) {
                obs.pending.push(record.clone());
            }
        }
    }
}
