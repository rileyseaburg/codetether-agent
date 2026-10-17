//! symbol reference during graph assembly.

use super::*;

impl GraphBuilder {
    pub(in crate::indexer) fn symbol_reference(&mut self, imported_symbol: String) -> String {
        let symbol_ref_id = self
            .symbol_ref_nodes
            .entry(imported_symbol.clone())
            .or_insert_with(|| {
                self.stats.symbol_reference_nodes += 1;
                let id = external_symbol_node_id(&imported_symbol);
                self.nodes.push(KnowledgeNode {
                    id: id.clone(),
                    kind: "symbol_ref".to_string(),
                    label: imported_symbol.clone(),
                    file_path: None,
                    language: None,
                    symbol_kind: None,
                    line: None,
                    external: true,
                });
                id
            })
            .clone();

        symbol_ref_id
    }
}
