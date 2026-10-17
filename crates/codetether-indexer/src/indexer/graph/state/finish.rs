//! finish during graph assembly.

use super::*;

impl GraphBuilder {
    pub(in crate::indexer) fn finish(mut self) -> KnowledgeGraph {
        self.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        self.edges.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| a.target.cmp(&b.target))
                .then_with(|| a.kind.cmp(&b.kind))
        });
        self.stats.edges = self.edges.len() as u64;

        KnowledgeGraph {
            version: KNOWLEDGE_GRAPH_VERSION,
            nodes: self.nodes,
            edges: self.edges,
            stats: self.stats,
        }
    }
}
