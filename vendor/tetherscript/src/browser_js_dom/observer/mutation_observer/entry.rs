//! Mutation observer entry and matching logic.

use super::super::mutation_types::{MutationObserverConfig, MutationRecord, MutationType};

pub type MutationCallback = Box<dyn FnMut(Vec<MutationRecord>)>;

pub struct MutationObserver {
    pub id: u64,
    pub target_node_id: u64,
    pub config: MutationObserverConfig,
    pub pending: Vec<MutationRecord>,
    pub(super) callback: MutationCallback,
}

impl MutationObserver {
    pub(super) fn new(
        id: u64,
        target: u64,
        config: MutationObserverConfig,
        cb: MutationCallback,
    ) -> Self {
        Self {
            id,
            target_node_id: target,
            config,
            pending: vec![],
            callback: cb,
        }
    }

    pub(super) fn matches(&self, record: &MutationRecord, ancestors: &[u64]) -> bool {
        if record.target_node_id != self.target_node_id
            && !(self.config.subtree && ancestors.contains(&self.target_node_id))
        {
            return false;
        }
        match record.type_ {
            MutationType::ChildList => self.config.child_list,
            MutationType::Attributes => self.matches_attribute(record),
            MutationType::CharacterData => self.config.character_data,
        }
    }

    fn matches_attribute(&self, record: &MutationRecord) -> bool {
        self.config.attributes
            && (self.config.attribute_filter.is_empty()
                || record
                    .attribute_name
                    .as_ref()
                    .is_some_and(|n| self.config.attribute_filter.contains(n)))
    }
}
