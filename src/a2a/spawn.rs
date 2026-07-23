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

use crate::a2a::lan;
use crate::a2a::mdns::{self, DiscoveredPeer};
use crate::a2a::server::A2AServer;
use crate::a2a::{intro::send_intro, local_identity};
use crate::bus::AgentBus;
use crate::cli::SpawnArgs;
use anyhow::{Context, Result};
use axum::Router;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

#[path = "spawn/agent_card.rs"]
mod agent_card;
#[path = "spawn/card_description.rs"]
mod card_description;
#[path = "spawn/handle.rs"]
mod handle;
#[path = "spawn/lan_address.rs"]
mod lan_address;
pub use handle::A2APeerHandle;
#[path = "spawn/mdns_addresses.rs"]
mod mdns_addresses;
#[path = "spawn/mdns_probe.rs"]
mod mdns_probe;
#[path = "spawn/peer_probe.rs"]
mod peer_probe;
#[path = "spawn/peer_url.rs"]
mod peer_url;
use peer_probe::try_fetch_agent_card;
use peer_url::{collect_peers, normalize_base_url, peer_candidates};

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

struct A2APreparation {
    listener: tokio::net::TcpListener,
    router: Router,
    discovery_task: tokio::task::JoinHandle<()>,
    mdns_intake_task: Option<tokio::task::JoinHandle<()>>,
    lan_tasks: Vec<tokio::task::JoinHandle<()>>,
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
        "0.0.0.0" | "::" | "[::]" => {
            lan_address::first_lan_ipv4().unwrap_or_else(|| "127.0.0.1".to_string())
        }
        other => other.to_string(),
    };
    let public_url = opts
        .public_url
        .clone()
        .unwrap_or_else(|| format!("http://{public_host}:{effective_port}"));
    let public_url = normalize_base_url(&public_url)?;

    // 4. Build card.
    let card = agent_card::build(&agent_name, &public_url, opts.description.as_deref());

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
    // Advertise the single concrete address used in the agent card. Auto
    // detection can publish every container bridge on busy hosts, causing
    // peers to waste time probing endpoints that were never externally useful.
    let mdns_bind_addrs = mdns_addresses::concrete(&public_host, local_addr.ip());
    let mut lan_tasks = Vec::new();
    let (mdns_handle, mdns_intake_task) = if opts.mdns && !mdns_bind_addrs.is_empty() {
        let collaboration_token = crate::a2a::collaboration_token::local().to_string();
        let (peer_tx, peer_rx) = mpsc::channel::<DiscoveredPeer>(64);
        match lan::announce_and_listen(
            agent_name.clone(),
            public_url.clone(),
            collaboration_token.clone(),
            peer_tx.clone(),
        )
        .await
        {
            Ok(tasks) => lan_tasks = tasks,
            Err(error) => tracing::warn!(%error, "A2A LAN broadcast discovery unavailable"),
        }
        match mdns::announce_and_browse(
            &agent_name,
            effective_port,
            mdns_bind_addrs,
            &collaboration_token,
            peer_tx,
        ) {
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

    let router: Router = A2AServer::with_bus(card, Arc::clone(&bus)).router();

    Ok(A2APreparation {
        listener,
        router,
        discovery_task,
        mdns_intake_task,
        lan_tasks,
        mdns_handle,
        agent_name,
        bind_addr,
        public_url,
    })
}

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
    let liveness = Arc::new(Mutex::new(crate::a2a::peer_liveness::PeerLiveness::new()));
    let in_flight = Arc::new(Mutex::new(HashSet::<String>::new()));

    // Background sweep: age out peers that have stopped re-announcing so the
    // TUI registry / UI counts don't show ghosts. mDNS re-resolves live peers
    // periodically (refreshing their last-seen); a peer that goes away stops
    // refreshing and is expired after the TTL. The guard aborts the task on drop.
    let _expire_guard =
        crate::a2a::mdns_liveness::spawn_expire_loop(Arc::clone(&bus), Arc::clone(&liveness));

    while let Some(peer) = peer_rx.recv().await {
        // Cheap pre-check by instance name: skip only the duplicate
        // per-interface announcements within the refresh window. Older
        // sightings fall through so the peer re-heartbeats.
        let recently_seen = {
            let l = liveness.lock().await;
            l.recently_seen(
                &peer.instance_name,
                crate::a2a::mdns_liveness::MDNS_PEER_REFRESH_INTERVAL,
            )
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
            handle_mdns_peer(bus, liveness, self_url, agent_name, peer, auto_introduce).await;
            in_flight.lock().await.remove(&instance_name);
        });
    }
}

/// Resolve a discovered peer, then register and optionally introduce it.
async fn handle_mdns_peer(
    bus: Arc<AgentBus>,
    liveness: Arc<Mutex<crate::a2a::peer_liveness::PeerLiveness>>,
    self_url: Arc<String>,
    agent_name: String,
    peer: DiscoveredPeer,
    auto_introduce: bool,
) {
    let Some((endpoint, card)) = mdns_probe::resolve(&peer, &self_url, &agent_name).await else {
        return;
    };

    let is_new = {
        let mut l = liveness.lock().await;
        l.record(&peer.instance_name, &card.name)
    };

    bus.registry.register(card.clone());
    crate::a2a::peer_route::register(&card, &endpoint, peer.token.clone());

    // Re-emit on every accepted resolution (not just first sight) so a peer
    // that was registered once but later dropped by a consumer reappears on
    // the next mDNS re-resolution instead of vanishing permanently.
    crate::a2a::mdns_liveness::emit_discovered(&bus, &card.name, &endpoint);

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
            && let Err(error) = send_intro(
                &endpoint,
                &agent_name,
                self_url.as_str(),
                peer.token.as_deref(),
            )
            .await
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
    local_identity::activate(&agent_name);
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
        lan_tasks: prep.lan_tasks,
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
    let lan_tasks = prep.lan_tasks;
    let _mdns_handle = prep.mdns_handle; // dropped at end of fn → unregisters

    axum::serve(prep.listener, prep.router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Spawned A2A server failed")?;

    discovery_task.abort();
    if let Some(t) = mdns_intake_task {
        t.abort();
    }
    for task in lan_tasks {
        task.abort();
    }
    tracing::info!(agent = %agent_name, "Spawned A2A agent shut down");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Shutdown signal received");
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
            crate::a2a::peer_route::register(&card, &endpoint, None);

            if is_new {
                crate::a2a::mdns_liveness::emit_discovered(&bus, &card.name, &endpoint);
                tracing::info!(
                    agent = %agent_name,
                    peer_name = %card.name,
                    peer_url = %card.url,
                    endpoint = %endpoint,
                    "Discovered A2A peer"
                );

                if auto_introduce
                    && let Err(error) = send_intro(&endpoint, &agent_name, &self_url, None).await
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
