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
use codetether_a2a_http::peer_probe::{self as mdns_probe, try_fetch_agent_card};
pub use handle::A2APeerHandle;
mod accept_mdns;
mod background;
mod command;
mod intake;
mod naming;
mod options;
use accept_mdns::handle_mdns_peer;
pub use background::start_a2a_in_background;
use codetether_a2a_discovery::endpoint::{
    collect_peers, concrete, first_lan_ipv4, normalize_base_url, peer_candidates,
};
pub use command::run;
use intake::mdns_intake_loop;
pub use naming::auto_agent_name;
pub use options::SpawnOptions;

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
        "0.0.0.0" | "::" | "[::]" => first_lan_ipv4().unwrap_or_else(|| "127.0.0.1".to_string()),
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
    let mdns_bind_addrs = concrete(&public_host, local_addr.ip());
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
