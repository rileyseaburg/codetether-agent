//! Compose UDP fallback, mDNS advertisement and local peer intake.
use super::*;
pub(in crate::peer_startup) async fn start(
    opts: &SpawnOptions,
    bus: &Arc<AgentBus>,
    public_host: &str,
    local_addr: SocketAddr,
    effective_port: u16,
    public_url: &String,
    agent_name: &String,
    hooks: StartupHooks,
) -> (
    Option<mdns::MdnsHandle>,
    Option<JoinHandle<()>>,
    Vec<JoinHandle<()>>,
) {
    // 6. mDNS announce + browse (true P2P, default on).
    //
    // Advertise the single concrete address used in the agent card. Auto
    // detection can publish every container bridge on busy hosts, causing
    // peers to waste time probing endpoints that were never externally useful.
    let mdns_bind_addrs = concrete(&public_host, local_addr.ip());
    let mut lan_tasks = Vec::new();
    let (mdns_handle, mdns_intake_task) = if opts.mdns && !mdns_bind_addrs.is_empty() {
        let collaboration_token = collaboration_token::local().to_string();
        let (peer_tx, peer_rx) = mpsc::channel::<DiscoveredPeer>(64);
        lan_tasks = start_lan(agent_name, public_url, &collaboration_token, &peer_tx).await;
        start_mdns(
            bus,
            agent_name,
            effective_port,
            mdns_bind_addrs,
            &collaboration_token,
            peer_tx,
            peer_rx,
            public_url,
            opts.auto_introduce,
            hooks,
        )
    } else {
        (None, None)
    };

    (mdns_handle, mdns_intake_task, lan_tasks)
}
