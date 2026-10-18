use crate::{
    WorkerContext, pending::fetch_pending_tasks, workspace_sync::sync_workspaces_from_server,
    workspace_sync_start::start_workspace_sync,
};
use codetether_a2a_worker_core::initialization::bootstrap::Host;
pub(super) struct Native<'a, D> {
    pub context: &'a WorkerContext,
    pub dispatcher: std::marker::PhantomData<fn() -> D>,
}
impl<D: crate::pending::Dispatcher> Host for Native<'_, D> {
    type SyncHandle = tokio::task::JoinHandle<()>;
    async fn register(&self) -> anyhow::Result<()> {
        crate::register_current_worker(self.context).await
    }
    async fn sync_workspaces(&self) -> anyhow::Result<()> {
        sync_workspaces_from_server(
            &self.0.task_runtime.client,
            &self.0.server,
            &self.0.args.token,
            &self.0.shared_codebases,
        )
        .await
    }
    async fn fetch_pending(&self) -> anyhow::Result<()> {
        fetch_pending_tasks(&self.0.task_runtime).await
    }
    fn start_workspace_sync(&self) -> Self::SyncHandle {
        start_workspace_sync(
            self.0.task_runtime.client.clone(),
            self.0.server.clone(),
            self.0.args.token.clone(),
            self.0.shared_codebases.clone(),
        )
    }
}
