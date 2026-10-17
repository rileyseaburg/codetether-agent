//! Mutable graph assembly state shared by focused construction passes.

use super::*;

#[derive(Default)]
pub(in crate::indexer) struct GraphBuilder {
    nodes: Vec<KnowledgeNode>,
    edges: Vec<KnowledgeEdge>,
    seen_edges: HashSet<(String, String, String)>,
    symbol_index: HashMap<String, Vec<String>>,
    module_nodes: HashMap<String, String>,
    symbol_ref_nodes: HashMap<String, String>,
    stats: KnowledgeGraphStats,
}

mod definitions;
mod finish;
mod modules;
mod symbol_reference;
mod symbols;
