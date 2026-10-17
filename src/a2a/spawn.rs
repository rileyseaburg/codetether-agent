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
//! - For each seed, build candidates via [`peer_candidates`](codetether_a2a_discovery::endpoint::peer_candidates) (tries `seed`
//!   and `seed/a2a` unless the seed already ends in `/a2a`).
//! - First successful agent-card fetch wins; the card is registered in
//!   `bus.registry`.
//! - On *first* sighting of a given endpoint: emit
//!   `Discovered A2A peer` and (if `auto_introduce`) call
//!   [`send_intro`](crate::a2a::intro::send_intro) over the A2A client.
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
use crate::a2a::local_identity;
use crate::a2a::mdns::{self, DiscoveredPeer};
use crate::a2a::server::A2AServer;
use crate::bus::AgentBus;
use crate::cli::SpawnArgs;
use anyhow::{Context, Result};
use axum::Router;
use std::sync::Arc;
use tokio::sync::mpsc;

#[path = "spawn/agent_card.rs"]
mod agent_card;
pub use codetether_a2a_http::peer_handle::A2APeerHandle;
mod background;
mod command;
mod options;
pub use background::start_a2a_in_background;
use codetether_a2a_discovery::endpoint::{
    collect_peers, concrete, first_lan_ipv4, normalize_base_url,
};
pub use codetether_a2a_discovery::naming::auto_agent_name;
use codetether_a2a_http::peer_intake::{discovery_loop, mdns_intake_loop};
pub use command::run;
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
        crate::a2a::intro::ledger::ledger_path,
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
                    crate::a2a::intro::ledger::ledger_path,
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
