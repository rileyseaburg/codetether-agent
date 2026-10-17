//! Start additive polling of configured peer seeds.
use super::*;
pub(super) fn start(
    opts: &SpawnOptions,
    bus: &Arc<AgentBus>,
    public_url: &String,
    agent_name: &String,
    hooks: StartupHooks,
) -> JoinHandle<()> {
    // 5. Spawn explicit-seed discovery loop (additive to mDNS-discovered
    //    peers — useful for cross-LAN/WAN seeds where multicast doesn't
    //    route).
    let peers = collect_peers(&opts.peer, &public_url);
    if !peers.is_empty() {
        tracing::info!(target: "codetether_agent::a2a::spawn",
            agent = %agent_name,
            peer_count = peers.len(),
            "A2A peer started with explicit --peer seeds (additive to mDNS)"
        );
    }
    tokio::spawn(discovery_loop(
        Arc::clone(&bus),
        peers,
        public_url.clone(),
        agent_name.clone(),
        opts.discovery_interval_secs.max(5),
        opts.auto_introduce,
        hooks.ledger_path,
    ))
}
