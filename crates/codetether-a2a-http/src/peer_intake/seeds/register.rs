//! Endpoint deduplication, publication and introduction for configured seeds.
use super::*;

pub(super) async fn register(
    bus: &Arc<AgentBus>,
    discovered: &Mutex<HashSet<String>>,
    endpoint: String,
    card: AgentCard,
    agent_name: &str,
    self_url: &str,
    auto_introduce: bool,
    resolve_path: fn() -> Option<PathBuf>,
) {
    // Dedup by endpoint only: card names embed the peer PID and
    // churn on every restart, which caused intro storms. The
    // persistent ledger in `intro::send` handles cross-restart
    // dedup on our side.
    let peer_id = endpoint.trim_end_matches('/').to_string();
    let is_new = {
        let mut known = discovered.lock().await;
        known.insert(peer_id)
    };

    bus.registry.register(card.clone());
    routes::register(&card, &endpoint, None);

    if is_new {
        presence::emit_discovered(bus, &card.name, &endpoint);
        tracing::info!(target: "codetether_agent::a2a::spawn",
            agent = %agent_name,
            peer_name = %card.name,
            peer_url = %card.url,
            endpoint = %endpoint,
            "Discovered A2A peer"
        );

        if auto_introduce
            && let Err(error) =
                send_intro(&endpoint, &agent_name, &self_url, None, resolve_path).await
        {
            tracing::warn!(target: "codetether_agent::a2a::spawn",
                agent = %agent_name,
                peer = %endpoint,
                error = %error,
                "Auto-intro message failed"
            );
        }
    }
}
