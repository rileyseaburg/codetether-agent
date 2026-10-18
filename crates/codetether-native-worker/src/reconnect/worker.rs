use super::{Native, mode::Mode};
use crate::{
    pending::{Dispatcher, fetch_pending_tasks},
    register_current_worker,
    stream::connect_stream,
};
use codetether_a2a_stream::{ingest::StreamDisconnectReason, worker_reconnect::Worker};
use tokio::{sync::mpsc, task::JoinHandle};
impl<D: Dispatcher, S: Mode> Worker for Native<'_, D, S> {
    async fn codebases(&self) -> Vec<String> {
        self.context.shared_codebases.lock().await.clone()
    }
    async fn register(&self) -> anyhow::Result<()> {
        register_current_worker(self.context).await
    }
    async fn fetch_pending(&self) -> anyhow::Result<()> {
        fetch_pending_tasks::<D>(&self.context.task_runtime).await
    }
    fn heartbeat(&self) -> JoinHandle<()> {
        codetether_a2a_http::worker::start_connection_heartbeat(self.context, |args| {
            args.token.clone()
        })
    }
    async fn connect(
        &self,
        codebases: &[String],
        notifications: Option<mpsc::Receiver<String>>,
    ) -> anyhow::Result<StreamDisconnectReason> {
        connect_stream::<D>(
            &self.context.task_runtime,
            &self.context.name,
            codebases,
            notifications,
            self.state.server_state(),
        )
        .await
    }
}
