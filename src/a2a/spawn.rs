//! Spawn an autonomous A2A agent runtime with auto peer discovery.

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

/// Run the spawn command.
pub async fn run(args: SpawnArgs) -> Result<()> {
    let bind_addr = format!("{}:{}", args.hostname, args.port);
    let public_url = args
        .public_url
        .clone()
        .unwrap_or_else(|| format!("http://{bind_addr}"));
    let public_url = normalize_base_url(&public_url)?;

    let agent_name = args
        .name
        .clone()
        .unwrap_or_else(|| format!("spawned-agent-{}", std::process::id()));

    let mut card = A2AServer::default_card(&public_url);
    card.name = agent_name.clone();
    if let Some(description) = args.description.clone() {
        card.description = description;
    }

    let bus = AgentBus::new().into_arc();
    bus.registry.register(card.clone());
    let handle = bus.handle(&agent_name);
    let capabilities = card.skills.iter().map(|skill| skill.id.clone()).collect();
    handle.announce_ready(capabilities);

    let peers = collect_peers(&args.peer, &public_url);
    if peers.is_empty() {
        tracing::info!(
            agent = %agent_name,
            "Spawned without peer seeds; pass --peer or CODETETHER_A2A_PEERS for discovery"
        );
    } else {
        tracing::info!(
            agent = %agent_name,
            peer_count = peers.len(),
            "Spawned with peer discovery seeds"
        );
    }

    let discovery_task = tokio::spawn(discovery_loop(
        Arc::clone(&bus),
        peers,
        normalize_base_url(&public_url)?,
        agent_name.clone(),
        args.discovery_interval_secs.max(5),
        args.auto_introduce,
    ));

    let router: Router = A2AServer::new(card).router();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("Failed to bind spawn agent on {bind_addr}"))?;

    tracing::info!(
        agent = %agent_name,
        bind_addr = %bind_addr,
        public_url = %public_url,
        "Spawned A2A agent runtime"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Spawned A2A server failed")?;

    discovery_task.abort();
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

        if let Ok(normalized) = normalize_base_url(raw) {
            if normalized.trim_end_matches('/') != self_url {
                dedup.insert(normalized);
            }
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

                if auto_introduce {
                    if let Err(error) = send_intro(&endpoint, &agent_name, &self_url).await {
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
}

fn peer_candidates(seed: &str) -> Vec<String> {
    if seed.ends_with("/a2a") {
        return vec![seed.to_string()];
    }

    vec![seed.to_string(), format!("{seed}/a2a")]
}

async fn try_fetch_agent_card(endpoint: &str) -> Result<crate::a2a::types::AgentCard> {
    let client = A2AClient::new(endpoint);
    let card = client.get_agent_card().await?;
    Ok(card)
}

async fn send_intro(endpoint: &str, agent_name: &str, self_url: &str) -> Result<()> {
    let client = A2AClient::new(endpoint);
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
