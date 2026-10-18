//! Resolve the loaded session workspace using the native worker HTTP context.
use super::{Loader, SessionState, TaskContext, TaskTimeline, WorkerTaskRuntime};
use std::marker::PhantomData;
pub(super) struct Native<'a, L> {
    pub runtime: &'a WorkerTaskRuntime,
    pub loader: PhantomData<fn() -> L>,
}
impl<L: Loader> codetether_a2a_worker_core::preparation::session::Host for Native<'_, L> {
    type Loader = L;
    async fn resolve_workspace(
        &self,
        context: &TaskContext,
        session: &mut L::Session,
        timeline: &mut TaskTimeline,
    ) -> anyhow::Result<()> {
        codetether_a2a_http::workspace::resolve_session_workspace(
            &self.runtime.client,
            &self.runtime.server,
            &self.runtime.token,
            context,
            &mut session.metadata_mut().directory,
            timeline,
        )
        .await
    }
}
