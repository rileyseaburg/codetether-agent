//! Assemble definitions and imports, then return a deterministically sorted graph.

use super::*;

pub(in crate::indexer) fn build_knowledge_graph(
    files: Vec<AnalyzedFileKnowledge>,
) -> KnowledgeGraph {
    let mut graph = GraphBuilder::default();
    for file in &files {
        graph.add_definitions(file);
    }
    for file in files {
        graph.add_modules(&file.file_node.id, file.imported_modules);
        graph.add_symbols(&file.file_node.id, file.imported_symbols);
    }
    graph.finish()
}
