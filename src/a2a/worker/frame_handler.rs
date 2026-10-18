//! Native task hooks for shared resumable stream dispatch.
use super::{
    WorkerTaskRuntime, pending_tasks::poll_pending_tasks, task_dispatch::spawn_task_handler,
};
use crate::a2a::stream::{cursor::Cursor, frame::ParsedFrame};
use codetether_a2a_stream::dispatch::{self, Host};
struct Worker<'a>(&'a WorkerTaskRuntime);
impl Host for Worker<'_> {
    async fn dispatch(&self, task: &serde_json::Value) {
        spawn_task_handler(task, self.0).await;
    }
    async fn reconcile(&self) -> anyhow::Result<()> {
        poll_pending_tasks(self.0).await
    }
}
/// Process one frame with the shared cursor and dispatch policy.
pub(super) async fn handle_frame(
    frame: &ParsedFrame,
    runtime: &WorkerTaskRuntime,
    cursor: &mut Cursor,
) {
    dispatch::handle_frame(frame, &Worker(runtime), cursor).await;
}
