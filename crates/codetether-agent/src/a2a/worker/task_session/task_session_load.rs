//! Native loading operations for shared task-session selection.
use crate::session::Session;
pub(in crate::a2a::worker) struct Native;
impl codetether_a2a_worker_core::preparation::session::Loader for Native {
    type Session = Session;
    async fn resume(id: &str) -> anyhow::Result<Session> {
        Session::resume(id).await
    }
    async fn resolve(context_id: Option<&str>) -> anyhow::Result<Session> {
        crate::a2a::session_resolve::resolve_session(context_id).await
    }
}
#[cfg(test)]
async fn load_ids(
    resume_session_id: Option<&str>,
    context_id: Option<&str>,
    require_resume: bool,
) -> anyhow::Result<Session> {
    codetether_a2a_worker_core::preparation::session::load_ids::<Native>(
        resume_session_id,
        context_id,
        require_resume,
    )
    .await
}
#[cfg(test)]
#[path = "task_session_load_tests.rs"]
mod tests;
