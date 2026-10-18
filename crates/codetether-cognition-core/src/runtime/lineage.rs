//! Application read lock for shared lineage graph construction.
use super::CognitionRuntime;
use crate::status::LineageGraph;

impl<T, R> CognitionRuntime<T, R> {
    /// Build the lineage graph from current persona state.
    ///
    /// Nodes and roots are sorted by persona ID for stable output.
    pub async fn lineage_graph(&self) -> LineageGraph {
        let personas = self.personas.read().await;
        crate::persona::graph::lineage_graph(&personas)
    }
}
