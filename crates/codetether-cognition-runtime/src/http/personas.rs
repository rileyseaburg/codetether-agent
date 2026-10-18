//! Native cognition personas bindings.
use crate::{CognitionRuntime, Host};
use codetether_cognition_core::CreatePersonaRequest;
use codetether_cognition_core::PersonaRuntimeState;
use codetether_cognition_core::ReapPersonaRequest;
use codetether_cognition_core::SpawnPersonaRequest;
use codetether_cognition_core::status::LineageGraph;
use codetether_cognition_core::status::ReapPersonaResponse;
impl<H: Host> codetether_cognition_api::http::Personas for CognitionRuntime<H>
where
    H::Thinker: Send + Sync,
    H::Registry: Send + Sync,
{
    async fn create_persona(
        &self,
        req: CreatePersonaRequest,
    ) -> anyhow::Result<PersonaRuntimeState> {
        CognitionRuntime::create_persona(self, req).await
    }
    async fn spawn_child(
        &self,
        id: &str,
        req: SpawnPersonaRequest,
    ) -> anyhow::Result<PersonaRuntimeState> {
        CognitionRuntime::spawn_child(self, id, req).await
    }
    async fn reap_persona(
        &self,
        id: &str,
        req: ReapPersonaRequest,
    ) -> anyhow::Result<ReapPersonaResponse> {
        CognitionRuntime::reap_persona(self, id, req).await
    }
    async fn get_persona(&self, id: &str) -> Option<PersonaRuntimeState> {
        CognitionRuntime::get_persona(self, id).await
    }
    async fn lineage_graph(&self) -> LineageGraph {
        CognitionRuntime::lineage_graph(self).await
    }
}
