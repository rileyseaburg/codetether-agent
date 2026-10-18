//! Runtime access for cognition personas.
use codetether_cognition_core::CreatePersonaRequest;
use codetether_cognition_core::PersonaRuntimeState;
use codetether_cognition_core::ReapPersonaRequest;
use codetether_cognition_core::SpawnPersonaRequest;
use codetether_cognition_core::status::LineageGraph;
use codetether_cognition_core::status::ReapPersonaResponse;
/// Runtime operations needed by personas HTTP handlers.
/// # Implementors
/// Native runtimes delegate these methods to their existing operations.
pub trait Personas: Send + Sync {
    /// Access the runtime's existing create persona operation.
    fn create_persona(
        &self,
        req: CreatePersonaRequest,
    ) -> impl std::future::Future<Output = anyhow::Result<PersonaRuntimeState>> + Send;
    /// Access the runtime's existing spawn child operation.
    fn spawn_child(
        &self,
        id: &str,
        req: SpawnPersonaRequest,
    ) -> impl std::future::Future<Output = anyhow::Result<PersonaRuntimeState>> + Send;
    /// Access the runtime's existing reap persona operation.
    fn reap_persona(
        &self,
        id: &str,
        req: ReapPersonaRequest,
    ) -> impl std::future::Future<Output = anyhow::Result<ReapPersonaResponse>> + Send;
    /// Access the runtime's existing get persona operation.
    fn get_persona(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Option<PersonaRuntimeState>> + Send;
    /// Access the runtime's existing lineage graph operation.
    fn lineage_graph(&self) -> impl std::future::Future<Output = LineageGraph> + Send;
}
