use super::Output;
use crate::{
    outcome::TaskInner, preparation::session::SessionState, task_config::TaskContext,
    task_timeline::TaskTimeline,
};
use codetether_provenance::ClaimProvenance;
use std::future::Future;
/// Native task-session preparation and agent execution.
/// # Implementors
/// Adapters retain concrete sessions and their title/execution APIs.
pub trait Sessions: Sync {
    /// Concrete session with shared metadata access.
    type Session: SessionState;
    /// Prepare a claimed task's session and normalized agent type.
    /// # Errors
    /// Propagates session, workspace or repository setup errors.
    fn prepare(
        &self,
        task_id: &str,
        context: &TaskContext,
        timeline: &mut TaskTimeline,
        claim: &ClaimProvenance,
        keys: Option<serde_json::Value>,
    ) -> impl Future<Output = anyhow::Result<(Self::Session, String)>> + Send;
    /// Preseed a missing session title using the task's human-readable title.
    fn preseed(&self, session: &mut Self::Session, title: &str);
    /// Execute the selected agent with its output observer.
    /// # Errors
    /// Returns native agent execution errors for task normalization.
    fn execute(
        &self,
        session: &mut Self::Session,
        context: &TaskContext,
        agent: &str,
        output: Output,
    ) -> impl Future<Output = TaskInner> + Send;
}
