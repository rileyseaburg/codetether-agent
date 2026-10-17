//! Concurrent intake and refresh of discovered peers.
use super::*;

/// Drain peers discovered over mDNS, fetch their cards, register them, and
/// optionally send the same auto-intro the explicit-seed loop sends.
///
/// Each discovered peer is processed on its own tokio task so a slow or
/// unreachable peer cannot block the rest of the queue. Dedup is keyed
/// by **agent name** (not by endpoint URL), so an agent reachable via
/// multiple network interfaces — common when binding `0.0.0.0` on a
/// multi-homed host — only triggers one discovery log line and one
/// auto-intro across the lifetime of the process. The known-set is
/// also pre-checked by mDNS instance name before any fetch is issued,
/// which short-circuits the duplicate announcements that mdns-sd
/// emits per interface.
///
/// # Arguments
///
/// Supply the bus, discovery receiver, local URL and name, introduction flag,
/// and a function that resolves the current introduction-ledger path.
///
/// # Returns
///
/// Returns when all discovery senders have closed; the expiry guard then stops.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_intake::mdns_intake_loop;
/// let _intake = mdns_intake_loop;
/// ```
pub async fn mdns_intake_loop(
    bus: Arc<AgentBus>,
    mut peer_rx: mpsc::Receiver<DiscoveredPeer>,
    self_url: String,
    agent_name: String,
    auto_introduce: bool,
    resolve_path: fn() -> Option<PathBuf>,
) {
    let self_url = Arc::new(self_url.trim_end_matches('/').to_string());
    let liveness = Arc::new(Mutex::new(PeerLiveness::new()));
    let in_flight = Arc::new(Mutex::new(HashSet::<String>::new()));

    // Background sweep: age out peers that have stopped re-announcing so the
    // TUI registry / UI counts don't show ghosts. mDNS re-resolves live peers
    // periodically (refreshing their last-seen); a peer that goes away stops
    // refreshing and is expired after the TTL. The guard aborts the task on drop.
    let _expire_guard = presence::spawn_expire_loop(Arc::clone(&bus), Arc::clone(&liveness));

    while let Some(peer) = peer_rx.recv().await {
        // Cheap pre-check by instance name: skip only the duplicate
        // per-interface announcements within the refresh window. Older
        // sightings fall through so the peer re-heartbeats.
        let recently_seen = {
            let l = liveness.lock().await;
            l.recently_seen(&peer.instance_name, presence::MDNS_PEER_REFRESH_INTERVAL)
        };
        if recently_seen {
            continue;
        }
        if !in_flight.lock().await.insert(peer.instance_name.clone()) {
            continue;
        }

        // Spawn one task per discovered peer so a slow/unreachable peer
        // can't block the next event in the queue.
        let bus = Arc::clone(&bus);
        let liveness = Arc::clone(&liveness);
        let in_flight = Arc::clone(&in_flight);
        let self_url = Arc::clone(&self_url);
        let agent_name = agent_name.clone();
        tokio::spawn(async move {
            let instance_name = peer.instance_name.clone();
            handle_mdns_peer(
                bus,
                liveness,
                self_url,
                agent_name,
                peer,
                auto_introduce,
                resolve_path,
            )
            .await;
            in_flight.lock().await.remove(&instance_name);
        });
    }
}
