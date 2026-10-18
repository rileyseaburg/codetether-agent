use crate::{task_config::TaskContext, task_timeline::TaskTimeline};
use codetether_provenance::ClaimProvenance;
use codetether_session_metadata::SessionMetadata;
use std::future::Future;
/// Session metadata and provenance mutations needed during preparation.
/// # Implementors
/// Native sessions retain the implementation of agent/provenance updates.
pub trait SessionState: Send {
    /// Session identifier for timeline checkpoints.
    fn id(&self) -> &str;
    /// Current metadata used by Git setup and checkpoints.
    fn metadata(&self) -> &SessionMetadata;
    /// Metadata updated by accepted task credentials and model selection.
    fn metadata_mut(&mut self) -> &mut SessionMetadata;
    /// Set the owning agent and its provenance.
    fn set_agent(&mut self, name: String);
    /// Attach the accepted task-claim provenance.
    fn attach_claim(&mut self, claim: &ClaimProvenance);
}
/// Native persistence operations for task session resolution.
/// # Implementors
/// Loaders preserve native resume and context-session lookup semantics.
pub trait Loader {
    /// Session type returned by loading.
    type Session: SessionState;
    /// Resume an existing session by ID.
    /// # Errors
    /// Propagates session lookup or hydration failures.
    fn resume(id: &str) -> impl Future<Output = anyhow::Result<Self::Session>> + Send;
    /// Resolve or create the task's context session.
    /// # Errors
    /// Propagates native context-session resolution failures.
    fn resolve(
        context_id: Option<&str>,
    ) -> impl Future<Output = anyhow::Result<Self::Session>> + Send;
}
/// Workspace resolution used before preparing task metadata.
/// # Implementors
/// Worker runtime adapters retain server-aware workspace selection.
pub trait Host: Sync {
    /// Native persistence implementation.
    type Loader: Loader;
    /// Resolve workspace metadata and its timeline checkpoints.
    /// # Errors
    /// Propagates native workspace-resolution failures.
    fn resolve_workspace(
        &self,
        context: &TaskContext,
        session: &mut <Self::Loader as Loader>::Session,
        timeline: &mut TaskTimeline,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
