//! Application read lock for shared lineage graph construction.
use super::{CognitionRuntime, LineageGraph};

impl CognitionRuntime {
    /// Build the lineage graph from current persona state.
    ///
    /// Nodes and roots are sorted by persona ID for stable output.
    pub async fn lineage_graph(&self) -> LineageGraph {
        self.state.lineage_graph().await
    }
}
