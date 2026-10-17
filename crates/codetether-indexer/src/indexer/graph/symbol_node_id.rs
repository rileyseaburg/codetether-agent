//! symbol node id for codebase indexing.

pub(in crate::indexer) fn symbol_node_id(path: &str, name: &str, line: u32) -> String {
    format!("symbol:{path}:{line}:{name}")
}
