//! Persona lineage graph construction.

use std::collections::HashMap;

use super::{CognitionRuntime, LineageGraph, LineageNode};

impl CognitionRuntime {
    /// Build the lineage graph from current persona state.
    ///
    /// Nodes and roots are sorted by persona ID for stable output.
    pub async fn lineage_graph(&self) -> LineageGraph {
        let personas = self.personas.read().await;
        let mut children_by_parent: HashMap<String, Vec<String>> = HashMap::new();
        let mut roots = Vec::new();
        let mut total_edges = 0usize;

        for persona in personas.values() {
            match persona.identity.parent_id.clone() {
                Some(parent_id) => {
                    children_by_parent
                        .entry(parent_id)
                        .or_default()
                        .push(persona.identity.id.clone());
                    total_edges = total_edges.saturating_add(1);
                }
                None => roots.push(persona.identity.id.clone()),
            }
        }

        let mut nodes: Vec<LineageNode> = personas
            .values()
            .map(|persona| {
                let mut children = children_by_parent
                    .get(&persona.identity.id)
                    .cloned()
                    .unwrap_or_default();
                children.sort();
                LineageNode {
                    persona_id: persona.identity.id.clone(),
                    parent_id: persona.identity.parent_id.clone(),
                    children,
                    depth: persona.identity.depth,
                    status: persona.status,
                }
            })
            .collect();

        nodes.sort_by(|a, b| a.persona_id.cmp(&b.persona_id));
        roots.sort();
        LineageGraph {
            nodes,
            roots,
            total_edges,
        }
    }
}
