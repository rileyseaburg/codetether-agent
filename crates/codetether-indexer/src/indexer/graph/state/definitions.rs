//! definitions during graph assembly.

use super::*;

impl GraphBuilder {
    pub(in crate::indexer) fn add_definitions(&mut self, file: &AnalyzedFileKnowledge) {
        self.nodes.push(file.file_node.clone());
        self.stats.file_nodes += 1;

        for symbol in &file.symbol_nodes {
            self.nodes.push(symbol.clone());
            self.stats.symbol_nodes += 1;
            self.symbol_index
                .entry(symbol.label.clone())
                .or_default()
                .push(symbol.id.clone());
            push_knowledge_edge(
                &mut self.edges,
                &mut self.seen_edges,
                &file.file_node.id,
                &symbol.id,
                "defines",
            );
        }
    }
}
