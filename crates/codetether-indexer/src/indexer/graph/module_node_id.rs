//! module node id for codebase indexing.

pub(in crate::indexer) fn module_node_id(module: &str) -> String {
    format!("module:{module}")
}
