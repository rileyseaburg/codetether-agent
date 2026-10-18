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
            &self.context.task_runtime.client,
            &self.context.server,
            &self.context.args.token,
            &self.context.shared_codebases,
        )
        .await
    }
    async fn fetch_pending(&self) -> anyhow::Result<()> {
        fetch_pending_tasks::<D>(&self.context.task_runtime).await
    }
    fn start_workspace_sync(&self) -> Self::SyncHandle {
        start_workspace_sync(
            self.context.task_runtime.client.clone(),
            self.context.server.clone(),
            self.context.args.token.clone(),
            self.context.shared_codebases.clone(),
        )
    }
}
