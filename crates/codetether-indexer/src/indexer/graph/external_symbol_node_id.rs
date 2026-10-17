//! external symbol node id for codebase indexing.

pub(in crate::indexer) fn external_symbol_node_id(symbol: &str) -> String {
    format!("symbol-ref:{symbol}")
}
