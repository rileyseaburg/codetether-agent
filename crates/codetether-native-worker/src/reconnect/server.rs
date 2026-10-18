use super::Native;
use codetether_a2a_stream::worker_reconnect::Server;
use codetether_worker_http::WorkerServerState;
use tokio::sync::mpsc;
impl<D: crate::pending::Dispatcher> Server for Native<'_, D, &WorkerServerState> {
    fn server_url(&self) -> &str {
        &self.context.server
    }
    async fn set_notification_channel(&self, sender: mpsc::Sender<String>) {
        self.state.set_task_notification_channel(sender).await;
    }
    async fn set_connected(&self, connected: bool) {
        self.state.set_connected(connected).await;
    }
}
