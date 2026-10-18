//! Native task-session metadata and workspace bindings.
use super::super::{TaskContext, WorkerTaskRuntime, task_timeline};
use crate::{
    provenance::ClaimProvenance,
    session::{Session, SessionMetadata},
};
impl codetether_a2a_worker_core::preparation::session::SessionState for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
    fn set_agent(&mut self, name: String) {
        self.set_agent_name(name);
    }
    fn attach_claim(&mut self, claim: &ClaimProvenance) {
        self.attach_claim_provenance(claim);
    }
}
pub(super) struct Native<'a>(pub &'a WorkerTaskRuntime);
impl codetether_a2a_worker_core::preparation::session::Host for Native<'_> {
    type Loader = super::task_session_load::Native;
    async fn resolve_workspace(
        &self,
        context: &TaskContext,
        session: &mut Session,
        timeline: &mut task_timeline::TaskTimeline,
    ) -> anyhow::Result<()> {
        super::super::task_session_resolve::resolve_workspace(self.0, context, session, timeline)
            .await
    }
}
