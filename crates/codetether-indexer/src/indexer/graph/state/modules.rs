//! modules during graph assembly.

use super::*;

impl GraphBuilder {
    pub(in crate::indexer) fn add_modules(&mut self, file_id: &str, modules: Vec<String>) {
        for module in modules {
            let module_id = self
                .module_nodes
                .entry(module.clone())
                .or_insert_with(|| {
                    self.stats.module_nodes += 1;
                    let id = module_node_id(&module);
                    self.nodes.push(KnowledgeNode {
                        id: id.clone(),
                        kind: "module".to_string(),
                        label: module.clone(),
                        file_path: None,
                        language: None,
                        symbol_kind: None,
                        line: None,
                        external: true,
                    });
                    id
                })
                .clone();

            push_knowledge_edge(
                &mut self.edges,
                &mut self.seen_edges,
                file_id,
                &module_id,
                "imports_module",
            );
        }
    }
}
