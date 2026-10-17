//! symbols during graph assembly.

use super::*;

impl GraphBuilder {
    pub(in crate::indexer) fn add_symbols(&mut self, file_id: &str, symbols: Vec<String>) {
        for imported_symbol in symbols {
            let target_ids = self
                .symbol_index
                .get(&imported_symbol)
                .filter(|targets| !targets.is_empty() && targets.len() <= 8)
                .cloned();

            if let Some(target_ids) = target_ids {
                for target_id in target_ids {
                    push_knowledge_edge(
                        &mut self.edges,
                        &mut self.seen_edges,
                        file_id,
                        &target_id,
                        "imports_symbol",
                    );
                }
                continue;
            }

            let symbol_ref_id = self.symbol_reference(imported_symbol);

            push_knowledge_edge(
                &mut self.edges,
                &mut self.seen_edges,
                file_id,
                &symbol_ref_id,
                "imports_symbol",
            );
        }
    }
}
