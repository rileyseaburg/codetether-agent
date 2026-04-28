//! Spawn an autonomous A2A agent runtime with auto peer discovery.
//!
//! Stands up an [`A2AServer`] (Axum) on a configurable host:port, publishes an
//! agent card at `/.well-known/agent.json`, and runs a background discovery
//! loop that polls each `--peer` seed for its agent card. New peers are
//! registered in the local bus and (unless `--no-auto-introduce` is set) sent
//! a one-shot non-blocking `message/send` introduction.
//!
//! Each spawned process is a self-contained A2A node: there is no central
//! broker. Two `codetether spawn` instances pointing at each other form the
//! minimal mesh; `n` instances form a fully-connected mesh as long as the
//! seed graph is connected.
//!
//! # Lifecycle
//!
//! 1. Resolve identity (`--name`, bind addr, public URL).
//! 2. Build the default [`AgentCard`](crate::a2a::types::AgentCard) and let
//!    `--description` override the default text.
//! 3. Initialize [`AgentBus`], start the best-effort training-record S3 sink,
//!    register self in the registry, and announce-ready with the card's skill
//!    ids as capabilities.
//! 4. Spawn [`discovery_loop`] for `--peer` seeds.
//! 5. Bind the [`A2AServer::router`] and serve until SIGINT.
//! 6. On shutdown: abort discovery and exit cleanly.
//!
//! # Discovery
//!
//! Every `discovery_interval_secs` (clamped to ≥ 5):
//! - For each seed, build candidates via [`peer_candidates`] (tries `seed`
//!   and `seed/a2a` unless the seed already ends in `/a2a`).
//! - First successful agent-card fetch wins; the card is registered in
//!   `bus.registry`.
//! - On *first* sighting of a given `endpoint::card.name` pair: emit
//!   `Discovered A2A peer` and (if `auto_introduce`) call
//!   [`send_intro`] over the A2A client.
//! - Re-sightings re-register the card (refresh) but do not re-introduce.
//!
//! Outbound discovery and intro calls attach `CODETETHER_AUTH_TOKEN` as a
//! bearer if set.
//!
//! # See also
//!
//! Full prose documentation lives at `docs/a2a-spawn.md` (CLI reference,
//! HTTP/JSON-RPC API, curl recipes, cross-host setup, troubleshooting,
//! source map).

use crate::a2a::client::A2AClient;
use crate::a2a::mdns::{self, DiscoveredPeer};
use crate::a2a::server::A2AServer;
use crate::a2a::types::{Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part};
use crate::bus::AgentBus;
use crate::cli::SpawnArgs;
use anyhow::{Context, Result};
use axum::Router;
use reqwest::Url;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

/// Inputs for starting an A2A peer runtime.
///
/// Used by `codetether spawn` (one-shot CLI) and by `codetether tui` (which
/// runs the same A2A surface in the background alongside the interactive
/// session). Every field is optional / has an auto-pick default — the
/// "default by default by default" experience is `SpawnOptions::auto()`.
#[derive(Debug, Clone)]
pub struct SpawnOptions {
    /// Agent name. `None` → auto-pick `<host>-<repo>-<short-pid>`.
    pub name: Option<String>,
    /// Bind hostname. `0.0.0.0` is the zero-config default so mDNS can
    /// announce real interfaces. Use `127.0.0.1` for loopback-only peers.
    pub hostname: String,
    /// Bind port. `0` → OS picks an available port.
    pub port: u16,
    /// Public URL published in the agent card. `None` → derived from the
    /// effective bound address after binding.
    pub public_url: Option<String>,
    /// Optional card description.
    pub description: Option<String>,
    /// Explicit peer seeds — used in addition to mDNS discovery, mainly
    /// for cross-host setups where mDNS isn't routable.
    pub peer: Vec<String>,
    /// Polling interval for explicit peer seeds (mDNS is event-driven).
    /// Clamped to ≥ 5.
    pub discovery_interval_secs: u64,
    /// Send a non-blocking intro `message/send` on first sighting of a peer.
    pub auto_introduce: bool,
    /// Enable mDNS announce + browse (true P2P discovery, no central state).
    pub mdns: bool,
}

impl SpawnOptions {
    /// Zero-config defaults: wildcard host, auto port/name, mDNS, intro on.
    pub fn auto() -> Self {
        Self {
            name: None,
            hostname: "0.0.0.0".to_string(),
            port: 0,
            public_url: None,
            description: None,
            peer: Vec::new(),
            discovery_interval_secs: 15,
            auto_introduce: true,
            mdns: true,
        }
    }

    /// Materialize options from the `codetether spawn` CLI args.
    pub fn from_spawn_args(args: &SpawnArgs) -> Self {
        Self {
            name: args.name.clone(),
            hostname: args.hostname.clone(),
            port: args.port,
            public_url: args.public_url.clone(),
            description: args.description.clone(),
            peer: args.peer.clone(),
            discovery_interval_secs: args.discovery_interval_secs,
            auto_introduce: args.auto_introduce,
            mdns: args.mdns,
        }
    }
}

/// Auto-derive a stable, human-readable agent name from
/// `<short-host>-<repo-basename>-<short-pid>`. Falls back gracefully if any
/// component is unavailable.
pub fn auto_agent_name() -> String {
    let host_full = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "host".to_string());
    let host_short = host_full
        .split('.')
        .next()
        .unwrap_or(&host_full)
        .to_string();
    let repo = std::env::current_dir()
        .ok()
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("repo")
        .to_string();
    let pid = std::process::id();
    let short_pid = format!("{:04x}", pid & 0xffff);
    let raw = format!("{host_short}-{repo}-{short_pid}");
    sanitize_name(&raw)
}

fn sanitize_name(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

/// First non-loopback non-link-local IPv4 address on any UP interface.
/// Used to substitute for `0.0.0.0` in the public agent-card URL so the
/// card advertises a reachable address.
fn first_lan_ipv4() -> Option<String> {
    let ifaces = if_addrs::get_if_addrs().ok()?;
    ifaces
        .into_iter()
        .filter_map(|iface| {
            let std::net::IpAddr::V4(v4) = iface.ip() else {
                return None;
            };
            if v4.is_loopback() || v4.is_link_local() || v4.is_unspecified() {
                return None;
            }
            // Skip docker bridge and similar internal addresses by preferring
            // the actual host LAN interface — but we don't filter by name
            // since interface naming is platform-specific. The first match
            // is good enough; users with strict requirements can pass
            // `--public-url` explicitly.
            Some(v4.to_string())
        })
        .next()
}

/// Handle to A2A peer background tasks (server + discovery loop + mDNS).
///
/// Returned by [`start_a2a_in_background`]. The handle MUST be kept alive
/// (bound to a non-`_` variable, or stashed in app state) for the lifetime
/// of the peer — dropping it aborts every background task and unregisters
/// the mDNS service via the `Drop` impl below. `JoinHandle::drop()` on its
/// own only *detaches* tasks; the explicit `Drop` here is what actually
/// stops them.
pub struct A2APeerHandle {
    pub agent_name: String,
    pub bind_addr: String,
    pub public_url: String,
    server_task: tokio::task::JoinHandle<()>,
    discovery_task: tokio::task::JoinHandle<()>,
    mdns_intake_task: Option<tokio::task::JoinHandle<()>>,
    /// Held to keep the daemon alive; on drop unregisters the service.
    _mdns_handle: Option<mdns::MdnsHandle>,
}

impl A2APeerHandle {
    /// Abort all background tasks and shut down mDNS immediately.
    pub fn abort(self) {
        // Drop runs on `self` falling out of scope — it does the work.
        drop(self);
    }
}

impl Drop for A2APeerHandle {
    fn drop(&mut self) {
        self.server_task.abort();
        self.discovery_task.abort();
        if let Some(t) = self.mdns_intake_task.as_ref() {
            t.abort();
        }
        // _mdns_handle's own Drop unregisters the mDNS service.
    }
}

struct A2APreparation {
    listener: tokio::net::TcpListener,
    router: Router,
    discovery_task: tokio::task::JoinHandle<()>,
    mdns_intake_task: Option<tokio::task::JoinHandle<()>>,
    mdns_handle: Option<mdns::MdnsHandle>,
    agent_name: String,
    bind_addr: String,
    public_url: String,
}

/// Shared setup: bind listener (port 0 → OS-assigned), auto-pick name if
/// missing, build card, register on bus, spawn explicit-seed discovery
/// loop, and start mDNS announce + browse if enabled.
///
/// Bind happens first so we know the effective port before publishing a
/// URL. The TCP bind is the only step that can fail synchronously — and
/// because nothing is registered on the bus and no tasks are spawned
/// before bind succeeds, a bind failure leaves no leaked state.
async fn prepare_a2a(opts: SpawnOptions, bus: Arc<AgentBus>) -> Result<A2APreparation> {
    // 1. Bind first so we know the effective port (handles --port 0) AND so
    //    a bind failure short-circuits cleanly without leaving any
    //    background task or stale bus registry entry behind.
    let requested_bind_addr = format!("{}:{}", opts.hostname, opts.port);
    let listener = tokio::net::TcpListener::bind(&requested_bind_addr)
        .await
        .with_context(|| format!("Failed to bind A2A peer on {requested_bind_addr}"))?;
    let local_addr = listener
        .local_addr()
        .context("Failed to read local addr from listener")?;
    let effective_port = local_addr.port();
    let bind_addr = format!("{}:{}", opts.hostname, effective_port);

    // 2. Auto-pick name if not provided.
    let agent_name = opts.name.clone().unwrap_or_else(auto_agent_name);

    // 3. Derive public_url from effective bind addr if not supplied.
    //    A wildcard host (`0.0.0.0` / `::`) is not a usable URL — pick the
    //    first non-loopback IPv4 address on a real interface so the card
    //    advertises something callers can actually reach.
    let public_host = match opts.hostname.as_str() {
        "0.0.0.0" | "::" | "[::]" => first_lan_ipv4().unwrap_or_else(|| "127.0.0.1".to_string()),
        other => other.to_string(),
    };
    let public_url = opts
        .public_url
        .clone()
        .unwrap_or_else(|| format!("http://{public_host}:{effective_port}"));
    let public_url = normalize_base_url(&public_url)?;

    // 4. Build card.
    let mut card = A2AServer::default_card(&public_url);
    card.name = agent_name.clone();
    if let Some(description) = opts.description.clone() {
        card.description = description;
    }

    bus.registry.register(card.clone());
    let bus_handle = bus.handle(&agent_name);
    let capabilities = card.skills.iter().map(|skill| skill.id.clone()).collect();
    bus_handle.announce_ready(capabilities);

    // 5. Spawn explicit-seed discovery loop (additive to mDNS-discovered
    //    peers — useful for cross-LAN/WAN seeds where multicast doesn't
    //    route).
    let peers = collect_peers(&opts.peer, &public_url);
    if !peers.is_empty() {
        tracing::info!(
            agent = %agent_name,
            peer_count = peers.len(),
            "A2A peer started with explicit --peer seeds (additive to mDNS)"
        );
    }
    let discovery_task = tokio::spawn(discovery_loop(
        Arc::clone(&bus),
        peers,
        public_url.clone(),
        agent_name.clone(),
        opts.discovery_interval_secs.max(5),
        opts.auto_introduce,
    ));

    // 6. mDNS announce + browse (true P2P, default on).
    //
    // Pass concrete bound addrs to mdns-sd that match what the HTTP server
    // will actually accept — anything else risks advertising a URL no
    // peer can reach. For a wildcard bind (`0.0.0.0`) we leave the addr
    // list empty so mdns-sd's `enable_addr_auto` enumerates the real
    // interface IPs; for a loopback bind we pass loopback so we don't
    // advertise LAN addrs the server isn't bound to.
    let mdns_bind_addrs: Vec<std::net::IpAddr> = match opts.hostname.as_str() {
        "0.0.0.0" | "::" | "[::]" => vec![],
        other => other.parse::<std::net::IpAddr>().ok().into_iter().collect(),
    };
    let (mdns_handle, mdns_intake_task) = if opts.mdns {
        let (peer_tx, peer_rx) = mpsc::channel::<DiscoveredPeer>(64);
        match mdns::announce_and_browse(&agent_name, effective_port, mdns_bind_addrs, peer_tx) {
            Ok(handle) => {
                let intake = tokio::spawn(mdns_intake_loop(
                    Arc::clone(&bus),
                    peer_rx,
                    public_url.clone(),
                    agent_name.clone(),
                    opts.auto_introduce,
                ));
                (Some(handle), Some(intake))
            }
            Err(e) => {
                tracing::warn!(
                    agent = %agent_name,
                    error = %e,
                    "mDNS unavailable; falling back to explicit --peer seeds only"
                );
                (None, None)
            }
        }
    } else {
        (None, None)
    };

    let router: Router = A2AServer::new(card).router();

    Ok(A2APreparation {
        listener,
        router,
        discovery_task,
        mdns_intake_task,
        mdns_handle,
        agent_name,
        bind_addr,
        public_url,
    })
}

/// Per-peer card-fetch deadline. mDNS may resolve a peer via several IPs
/// (LAN, VPN, docker bridge, link-local v6); we want to fail fast on the
/// unreachable ones rather than blocking the queue.
const MDNS_PEER_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

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
async fn mdns_intake_loop(
    bus: Arc<AgentBus>,
    mut peer_rx: mpsc::Receiver<DiscoveredPeer>,
    self_url: String,
    agent_name: String,
    auto_introduce: bool,
) {
    let self_url = Arc::new(self_url.trim_end_matches('/').to_string());
    let known: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    while let Some(peer) = peer_rx.recv().await {
        // Cheap pre-check by instance name to skip the per-interface
        // duplicate announcements before we spawn anything.
        let already_known_by_instance = {
            let k = known.lock().await;
            k.contains(&peer.instance_name)
        };
        if already_known_by_instance {
            continue;
        }

        // Spawn one task per discovered peer so a slow/unreachable peer
        // can't block the next event in the queue.
        let bus = Arc::clone(&bus);
        let known = Arc::clone(&known);
        let self_url = Arc::clone(&self_url);
        let agent_name = agent_name.clone();
        tokio::spawn(async move {
            handle_mdns_peer(bus, known, self_url, agent_name, peer, auto_introduce).await;
        });
    }
}

/// Try each URL the resolver gave us until one yields a valid agent card,
/// then register and (optionally) intro. Each fetch attempt is bounded by
/// `MDNS_PEER_PROBE_TIMEOUT`. Multi-homed peers commonly resolve to
/// several addrs (LAN/VPN/docker); we cycle through them in order.
async fn handle_mdns_peer(
    bus: Arc<AgentBus>,
    known: Arc<Mutex<HashSet<String>>>,
    self_url: Arc<String>,
    agent_name: String,
    peer: DiscoveredPeer,
    auto_introduce: bool,
) {
    let mut resolved: Option<(String, crate::a2a::types::AgentCard)> = None;

    'urls: for url in &peer.urls {
        if url.trim_end_matches('/') == self_url.as_str() {
            continue;
        }
        for candidate in peer_candidates(url) {
            match tokio::time::timeout(MDNS_PEER_PROBE_TIMEOUT, try_fetch_agent_card(&candidate))
                .await
            {
                Ok(Ok(card)) => {
                    resolved = Some((candidate, card));
                    break 'urls;
                }
                Ok(Err(error)) => {
                    tracing::debug!(
                        agent = %agent_name,
                        peer = %candidate,
                        error = %error,
                        "mDNS peer probe failed"
                    );
                }
                Err(_) => {
                    tracing::debug!(
                        agent = %agent_name,
                        peer = %candidate,
                        timeout_secs = MDNS_PEER_PROBE_TIMEOUT.as_secs(),
                        "mDNS peer probe timed out"
                    );
                }
            }
        }
    }

    let Some((endpoint, card)) = resolved else {
        return;
    };

    let is_new = {
        let mut k = known.lock().await;
        // Insert both keys so future probes via either path short-circuit.
        let inserted = k.insert(card.name.clone());
        k.insert(peer.instance_name.clone());
        inserted
    };

    bus.registry.register(card.clone());

    if is_new {
        tracing::info!(
            agent = %agent_name,
            peer_name = %card.name,
            peer_url = %card.url,
            endpoint = %endpoint,
            via = "mdns",
            "Discovered A2A peer"
        );
        if auto_introduce
            && let Err(error) = send_intro(&endpoint, &agent_name, self_url.as_str()).await
        {
            tracing::warn!(
                agent = %agent_name,
                peer = %endpoint,
                error = %error,
                "Auto-intro message failed"
            );
        }
    }
}

/// Start an A2A peer runtime in the background, attached to the given bus.
///
/// Caller is responsible for keeping the returned [`A2APeerHandle`] alive
/// for as long as the peer should keep serving. Drop or call `abort()` on
/// shutdown. Used by the TUI on every launch (unless `--no-a2a` was set).
pub async fn start_a2a_in_background(
    opts: SpawnOptions,
    bus: Arc<AgentBus>,
) -> Result<A2APeerHandle> {
    let prep = prepare_a2a(opts, bus).await?;
    tracing::info!(
        agent = %prep.agent_name,
        bind_addr = %prep.bind_addr,
        public_url = %prep.public_url,
        mdns = prep.mdns_handle.is_some(),
        "A2A peer listening (background mode)"
    );
    let agent_name = prep.agent_name.clone();
    let bind_addr = prep.bind_addr.clone();
    let public_url = prep.public_url.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = axum::serve(prep.listener, prep.router).await {
            tracing::error!(error = %e, "A2A peer server task exited with error");
        }
    });
    Ok(A2APeerHandle {
        agent_name,
        bind_addr,
        public_url,
        server_task,
        discovery_task: prep.discovery_task,
        mdns_intake_task: prep.mdns_intake_task,
        _mdns_handle: prep.mdns_handle,
    })
}

/// Run the `codetether spawn` command. Owns its own bus and S3 sink, blocks
/// on ctrl_c with graceful shutdown.
pub async fn run(args: SpawnArgs) -> Result<()> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    let prep = prepare_a2a(SpawnOptions::from_spawn_args(&args), bus).await?;

    tracing::info!(
        agent = %prep.agent_name,
        bind_addr = %prep.bind_addr,
        public_url = %prep.public_url,
        mdns = prep.mdns_handle.is_some(),
        "Spawned A2A agent runtime"
    );

    let agent_name = prep.agent_name.clone();
    let discovery_task = prep.discovery_task;
    let mdns_intake_task = prep.mdns_intake_task;
    let _mdns_handle = prep.mdns_handle; // dropped at end of fn → unregisters

    axum::serve(prep.listener, prep.router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Spawned A2A server failed")?;

    discovery_task.abort();
    if let Some(t) = mdns_intake_task {
        t.abort();
    }
    tracing::info!(agent = %agent_name, "Spawned A2A agent shut down");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Shutdown signal received");
}

fn normalize_base_url(url: &str) -> Result<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    let normalized = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("http://{trimmed}")
    };

    let parsed = Url::parse(&normalized).with_context(|| format!("Invalid URL: {normalized}"))?;
    let mut cleaned = parsed.to_string();
    if cleaned.ends_with('/') {
        cleaned.pop();
    }
    Ok(cleaned)
}

fn collect_peers(raw_peers: &[String], self_url: &str) -> Vec<String> {
    let mut dedup = HashSet::new();
    let self_url = self_url.trim_end_matches('/');

    for raw in raw_peers {
        if raw.trim().is_empty() {
            continue;
        }

        if let Ok(normalized) = normalize_base_url(raw)
            && normalized.trim_end_matches('/') != self_url
        {
            dedup.insert(normalized);
        }
    }

    dedup.into_iter().collect()
}

async fn discovery_loop(
    bus: Arc<AgentBus>,
    peers: Vec<String>,
    self_url: String,
    agent_name: String,
    interval_secs: u64,
    auto_introduce: bool,
) {
    let discovered = Arc::new(Mutex::new(HashSet::<String>::new()));
    let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));

    loop {
        ticker.tick().await;

        for peer_seed in &peers {
            let candidates = peer_candidates(peer_seed);
            let mut discovered_card = None;

            for candidate in candidates {
                match try_fetch_agent_card(&candidate).await {
                    Ok(card) => {
                        discovered_card = Some((candidate, card));
                        break;
                    }
                    Err(error) => {
                        tracing::debug!(
                            agent = %agent_name,
                            peer = %candidate,
                            error = %error,
                            "Peer probe failed"
                        );
                    }
                }
            }

            let Some((endpoint, card)) = discovered_card else {
                continue;
            };

            let peer_id = format!("{}::{}", endpoint, card.name);
            let is_new = {
                let mut known = discovered.lock().await;
                known.insert(peer_id)
            };

            bus.registry.register(card.clone());

            if is_new {
                tracing::info!(
                    agent = %agent_name,
                    peer_name = %card.name,
                    peer_url = %card.url,
                    endpoint = %endpoint,
                    "Discovered A2A peer"
                );

                if auto_introduce
                    && let Err(error) = send_intro(&endpoint, &agent_name, &self_url).await
                {
                    tracing::warn!(
                        agent = %agent_name,
                        peer = %endpoint,
                        error = %error,
                        "Auto-intro message failed"
                    );
                }
            }
        }
    }
}

fn peer_candidates(seed: &str) -> Vec<String> {
    if seed.ends_with("/a2a") {
        return vec![seed.to_string()];
    }

    vec![seed.to_string(), format!("{seed}/a2a")]
}

async fn try_fetch_agent_card(endpoint: &str) -> Result<crate::a2a::types::AgentCard> {
    let mut client = A2AClient::new(endpoint);
    if let Ok(token) = std::env::var("CODETETHER_AUTH_TOKEN") {
        client = client.with_token(token);
    }
    let card = client.get_agent_card().await?;
    Ok(card)
}

async fn send_intro(endpoint: &str, agent_name: &str, self_url: &str) -> Result<()> {
    let mut client = A2AClient::new(endpoint);
    if let Ok(token) = std::env::var("CODETETHER_AUTH_TOKEN") {
        client = client.with_token(token);
    }
    let payload = MessageSendParams {
        message: Message {
            message_id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::User,
            parts: vec![Part::Text {
                text: format!(
                    "Hello from {agent_name} ({self_url}). I am online and available for A2A collaboration."
                ),
            }],
            context_id: None,
            task_id: None,
            metadata: std::collections::HashMap::new(),
            extensions: vec![],
        },
        configuration: Some(MessageSendConfiguration {
            accepted_output_modes: vec!["text/plain".to_string()],
            blocking: Some(false),
            history_length: Some(0),
            push_notification_config: None,
        }),
    };

    let _ = client.send_message(payload).await?;
    tracing::info!(peer = %endpoint, "Auto-intro message sent");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{collect_peers, peer_candidates};

    #[test]
    fn collect_peers_deduplicates_and_skips_self() {
        let peers = vec![
            "localhost:5000".to_string(),
            "http://localhost:5000/".to_string(),
            "http://localhost:5001".to_string(),
            "http://localhost:5002".to_string(),
        ];

        let mut out = collect_peers(&peers, "http://localhost:5001");
        out.sort();

        assert_eq!(
            out,
            vec![
                "http://localhost:5000".to_string(),
                "http://localhost:5002".to_string(),
            ]
        );
    }

    #[test]
    fn peer_candidates_adds_a2a_variant() {
        let out = peer_candidates("http://localhost:4096");
        assert_eq!(
            out,
            vec![
                "http://localhost:4096".to_string(),
                "http://localhost:4096/a2a".to_string()
            ]
        );

        let out2 = peer_candidates("http://localhost:4096/a2a");
        assert_eq!(out2, vec!["http://localhost:4096/a2a".to_string()]);
    }
}