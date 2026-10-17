//! Advertise mDNS and start intake only when advertisement succeeds.
use super::*;
pub(super) fn start(
    bus: &Arc<AgentBus>,
    agent_name: &String,
    effective_port: u16,
    mdns_bind_addrs: Vec<IpAddr>,
    collaboration_token: &String,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
    peer_rx: mpsc::Receiver<DiscoveredPeer>,
    public_url: &String,
    auto_introduce: bool,
    hooks: StartupHooks,
) -> (Option<mdns::MdnsHandle>, Option<JoinHandle<()>>) {
    match mdns::announce_and_browse(
        &agent_name,
        effective_port,
        mdns_bind_addrs,
        &collaboration_token,
        peer_tx,
        hooks.application_version,
    ) {
        Ok(handle) => {
            let intake = tokio::spawn(mdns_intake_loop(
                Arc::clone(&bus),
                peer_rx,
                public_url.clone(),
                agent_name.clone(),
                auto_introduce,
                hooks.ledger_path,
            ));
            (Some(handle), Some(intake))
        }
        Err(e) => {
            tracing::warn!(target: "codetether_agent::a2a::spawn",
                agent = %agent_name,
                error = %e,
                "mDNS unavailable; falling back to explicit --peer seeds only"
            );
            (None, None)
        }
    }
}
