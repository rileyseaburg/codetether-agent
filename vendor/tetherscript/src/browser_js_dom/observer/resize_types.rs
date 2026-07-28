//! Resize observer types.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResizeEntry {
    pub target_node_id: u64,
    pub content_width: i64,
    pub content_height: i64,
}
