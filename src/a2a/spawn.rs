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

/// Inputs for starting an A2A peer runtime.
///
/// Used both by `codetether spawn` (one-shot CLI) and by `codetether tui`
/// when its `--a2a-port` flag is set (background peer alongside the TUI).
#[derive(Debug, Clone)]
pub struct SpawnOptions {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub public_url: Option<String>,
    pub description: Option<String>,
    pub peer: Vec<String>,
    pub discovery_interval_secs: u64,
    pub auto_introduce: bool,
}

impl SpawnOptions {
    /// Materialize options from the `codetether spawn` CLI args, applying the
    /// same default-name policy (`spawned-agent-<pid>`).
    pub fn from_spawn_args(args: &SpawnArgs) -> Self {
        Self {
            name: args
                .name
                .clone()
                .unwrap_or_else(|| format!("spawned-agent-{}", std::process::id())),
            hostname: args.hostname.clone(),
            port: args.port,
            public_url: args.public_url.clone(),
            description: args.description.clone(),
            peer: args.peer.clone(),
            discovery_interval_secs: args.discovery_interval_secs,
            auto_introduce: args.auto_introduce,
        }
    }
}

/// Handle to A2A peer background tasks (server + discovery loop).
///
/// Returned by [`start_a2a_in_background`]. Drop the handle when the host
/// process is shutting down to abort both tasks; otherwise tokio will reap
/// them when the runtime exits.
pub struct A2APeerHandle {
    pub agent_name: String,
    pub bind_addr: String,
    pub public_url: String,
    server_task: tokio::task::JoinHandle<()>,
    discovery_task: tokio::task::JoinHandle<()>,
}

impl A2APeerHandle {
    /// Abort the background server and discovery tasks.
    pub fn abort(self) {
        self.server_task.abort();
        self.discovery_task.abort();
    }
}

struct A2APreparation {
    listener: tokio::net::TcpListener,
    router: Router,
    discovery_task: tokio::task::JoinHandle<()>,
    agent_name: String,
    bind_addr: String,
    public_url: String,
}

/// Shared setup: build card, register on bus, bind listener, spawn discovery.
///
/// The TCP bind is the only step that can fail synchronously, so the caller
/// gets a clean error before any tokio task is spawned.
async fn prepare_a2a(opts: SpawnOptions, bus: Arc<AgentBus>) -> Result<A2APreparation> {
    let bind_addr = format!("{}:{}", opts.hostname, opts.port);
    let public_url = opts
        .public_url
        .clone()
        .unwrap_or_else(|| format!("http://{bind_addr}"));
    let public_url = normalize_base_url(&public_url)?;

    let agent_name = opts.name.clone();
    let mut card = A2AServer::default_card(&public_url);
    card.name = agent_name.clone();
    if let Some(description) = opts.description.clone() {
        card.description = description;
    }

    bus.registry.register(card.clone());
    let bus_handle = bus.handle(&agent_name);
    let capabilities = card.skills.iter().map(|skill| skill.id.clone()).collect();
    bus_handle.announce_ready(capabilities);

    let peers = collect_peers(&opts.peer, &public_url);
    if peers.is_empty() {
        tracing::info!(
            agent = %agent_name,
            "A2A peer started without peer seeds; pass --peer or CODETETHER_A2A_PEERS for discovery"
        );
    } else {
        tracing::info!(
            agent = %agent_name,
            peer_count = peers.len(),
            "A2A peer started with peer discovery seeds"
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

    let router: Router = A2AServer::new(card).router();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("Failed to bind A2A peer on {bind_addr}"))?;

    Ok(A2APreparation {
        listener,
        router,
        discovery_task,
        agent_name,
        bind_addr,
        public_url,
    })
}

/// Start an A2A peer runtime in the background, attached to the given bus.
///
/// Caller is responsible for keeping the returned [`A2APeerHandle`] alive
/// for as long as the peer should keep serving. Drop or call `abort()` on
/// shutdown. Used by the TUI when `--a2a-port` is set.
pub async fn start_a2a_in_background(
    opts: SpawnOptions,
    bus: Arc<AgentBus>,
) -> Result<A2APeerHandle> {
    let prep = prepare_a2a(opts, bus).await?;
    tracing::info!(
        agent = %prep.agent_name,
        bind_addr = %prep.bind_addr,
        public_url = %prep.public_url,
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
        "Spawned A2A agent runtime"
    );

    axum::serve(prep.listener, prep.router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Spawned A2A server failed")?;

    prep.discovery_task.abort();
    tracing::info!(agent = %prep.agent_name, "Spawned A2A agent shut down");
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
