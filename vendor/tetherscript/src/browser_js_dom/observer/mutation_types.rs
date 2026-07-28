//! Mutation record and observer configuration types.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationType {
    ChildList,
    Attributes,
    CharacterData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutationRecord {
    pub type_: MutationType,
    pub target_node_id: u64,
    pub added_nodes: Vec<u64>,
    pub removed_nodes: Vec<u64>,
    pub previous_sibling: Option<u64>,
    pub next_sibling: Option<u64>,
    pub attribute_name: Option<String>,
    pub old_value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MutationObserverConfig {
    pub child_list: bool,
    pub attributes: bool,
    pub character_data: bool,
    pub subtree: bool,
    pub attribute_old_value: bool,
    pub character_data_old_value: bool,
    pub attribute_filter: Vec<String>,
}
