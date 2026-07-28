//! MutationRecord constructors.

use super::mutation_types::{MutationRecord, MutationType};

impl MutationRecord {
    pub fn child_list(target: u64, added: Vec<u64>, removed: Vec<u64>) -> Self {
        Self {
            type_: MutationType::ChildList,
            target_node_id: target,
            added_nodes: added,
            removed_nodes: removed,
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            old_value: None,
        }
    }

    pub fn attribute(target: u64, name: impl Into<String>, old: Option<String>) -> Self {
        Self {
            type_: MutationType::Attributes,
            target_node_id: target,
            added_nodes: vec![],
            removed_nodes: vec![],
            previous_sibling: None,
            next_sibling: None,
            attribute_name: Some(name.into()),
            old_value: old,
        }
    }
}
