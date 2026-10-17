//! Periodic polling of explicitly configured peer seeds.
use super::*;

/// Poll configured seeds and publish the first successful endpoint for each.
///
/// # Arguments
///
/// Supply the bus, seeds, local URL and name, polling interval, introduction
/// flag, and a function that resolves the current introduction-ledger path.
/// The caller must provide a nonzero polling interval.
///
/// # Returns
///
/// Runs until its owning task is cancelled.
///
/// # Panics
///
/// Panics when the polling interval is zero.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_intake::discovery_loop;
/// let _poll = discovery_loop;
/// ```
pub async fn discovery_loop(
    bus: Arc<AgentBus>,
    peers: Vec<String>,
    self_url: String,
    agent_name: String,
    interval_secs: u64,
    auto_introduce: bool,
    resolve_path: fn() -> Option<PathBuf>,
) {
    let discovered = Arc::new(Mutex::new(HashSet::<String>::new()));
    let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;

        for peer_seed in &peers {
            let discovered_card = probe(peer_seed, &agent_name).await;

            let Some((endpoint, card)) = discovered_card else {
                continue;
            };

            register(
                &bus,
                &discovered,
                endpoint,
                card,
                &agent_name,
                &self_url,
                auto_introduce,
                resolve_path,
            )
            .await;
        }
    }
}
