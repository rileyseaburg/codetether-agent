//! analyzed file knowledge for codebase indexing.

use super::*;

#[derive(Debug, Clone)]
pub(in crate::indexer) struct AnalyzedFileKnowledge {
    pub(in crate::indexer) file_node: KnowledgeNode,
    pub(in crate::indexer) symbol_nodes: Vec<KnowledgeNode>,
    pub(in crate::indexer) imported_modules: Vec<String>,
    pub(in crate::indexer) imported_symbols: Vec<String>,
}
