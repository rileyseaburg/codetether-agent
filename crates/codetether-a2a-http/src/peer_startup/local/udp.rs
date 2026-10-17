//! Start the UDP fallback before mDNS discovery.
use super::*;
pub(super) async fn start(
    agent_name: &String,
    public_url: &String,
    collaboration_token: &String,
    peer_tx: &mpsc::Sender<DiscoveredPeer>,
) -> Vec<JoinHandle<()>> {
    let mut lan_tasks = Vec::new();
    match lan::announce_and_listen(
        agent_name.clone(),
        public_url.clone(),
        collaboration_token.clone(),
        peer_tx.clone(),
    )
    .await
    {
        Ok(tasks) => lan_tasks = tasks,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::a2a::spawn",%error, "A2A LAN broadcast discovery unavailable")
        }
    }

    lan_tasks
}
