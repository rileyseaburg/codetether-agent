use super::{Native, mode::Mode};
use crate::a2a::worker::{
    connect_stream, connection_setup::start_connection_heartbeat, fetch_pending_tasks,
    register_current_worker,
};
use codetether_a2a_stream::{ingest::StreamDisconnectReason, worker_reconnect::Worker};
use tokio::{sync::mpsc, task::JoinHandle};
impl<S: Mode> Worker for Native<'_, S> {
    async fn codebases(&self) -> Vec<String> {
        self.context.shared_codebases.lock().await.clone()
    }
    async fn register(&self) -> anyhow::Result<()> {
        register_current_worker(self.context).await
    }
    async fn fetch_pending(&self) -> anyhow::Result<()> {
        fetch_pending_tasks(&self.context.task_runtime).await
    }
    fn heartbeat(&self) -> JoinHandle<()> {
        start_connection_heartbeat(self.context)
    }
    async fn connect(
        &self,
        codebases: &[String],
        notifications: Option<mpsc::Receiver<String>>,
    ) -> anyhow::Result<StreamDisconnectReason> {
        connect_stream(
            &self.context.task_runtime,
            &self.context.name,
            codebases,
            notifications,
            self.state.server_state(),
        )
        .await
    }
}
