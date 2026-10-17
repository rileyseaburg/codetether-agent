//! file node id for codebase indexing.

pub(in crate::indexer) fn file_node_id(path: &str) -> String {
    format!("file:{path}")
}
