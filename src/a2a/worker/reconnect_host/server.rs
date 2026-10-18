use super::Native;
use crate::worker_server::WorkerServerState;
use codetether_a2a_stream::worker_reconnect::Server;
use tokio::sync::mpsc;
impl Server for Native<'_, &WorkerServerState> {
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
