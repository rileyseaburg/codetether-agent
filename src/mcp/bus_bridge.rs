//! MCP-to-Bus bridge
//!
//! Connects to the HTTP server's `/v1/bus/stream` SSE endpoint and buffers
//! recent [`BusEnvelope`]s in a ring buffer.  The MCP server exposes this
//! data through tools (`bus_events`, `bus_status`) and resources
//! (`codetether://bus/events/recent`).
//!
//! The bridge runs as a background tokio task and reconnects automatically
//! on transient failures.

use crate::bus::BusEnvelope;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Maximum number of envelopes kept in the ring buffer.
const DEFAULT_BUFFER_SIZE: usize = 1_000;

/// Reconnect delay after a transient SSE failure.
const RECONNECT_DELAY: std::time::Duration = std::time::Duration::from_secs(3);

// ─── BusBridge ───────────────────────────────────────────────────────────

/// A read-only bridge from the HTTP bus SSE stream into the MCP process.
///
/// Call [`BusBridge::spawn`] to start the background reader, then query via
/// [`BusBridge::recent_events`] or [`BusBridge::status`].
#[derive(Debug)]
pub struct BusBridge {
    /// Ring buffer of recent envelopes (newest last).
    buffer: Arc<RwLock<VecDeque<BusEnvelope>>>,
    /// Whether the SSE reader is currently connected.
    connected: Arc<AtomicBool>,
    /// Total number of envelopes received since start.
    total_received: Arc<AtomicU64>,
    /// URL we connect to.
    bus_url: String,
    /// Optional bearer token for authenticated bus endpoints.
    auth_token: Option<String>,
    /// Max buffer capacity.
    capacity: usize,
    client: reqwest::Client,
}

impl BusBridge {
    /// Create a new bridge (does **not** start the background task).
    pub fn new(bus_url: String) -> Self {
        Self::with_auth(bus_url, None)
    }

    /// Create a new bridge with an optional bearer token.
    pub fn with_auth(bus_url: String, auth_token: Option<String>) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(VecDeque::with_capacity(DEFAULT_BUFFER_SIZE))),
            connected: Arc::new(AtomicBool::new(false)),
            total_received: Arc::new(AtomicU64::new(0)),
            bus_url,
            auth_token,
            capacity: DEFAULT_BUFFER_SIZE,
            client: reqwest::Client::new(),
        }
    }

    /// Spawn the SSE reader as a background tokio task.
    ///
    /// Returns `Self` wrapped in an `Arc` for sharing with tool handlers.
    pub fn spawn(self) -> Arc<Self> {
        let bridge = Arc::new(self);
        let bg = Arc::clone(&bridge);
        tokio::spawn(async move {
            bg.reader_loop().await;
        });
        bridge
    }

    /// Query recent events with optional topic filter and limit.
    pub async fn recent_events(
        &self,
        topic_filter: Option<&str>,
        limit: usize,
        since: Option<DateTime<Utc>>,
    ) -> Vec<BusEnvelope> {
        let buf = self.buffer.read().await;
        let mut events = buf
            .iter()
            .rev()
            .filter(|env| topic_filter.is_none_or(|filter| topic_matches(&env.topic, filter)))
            .filter(|env| since.is_none_or(|ts| env.timestamp >= ts))
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        events.reverse();
        events
    }

    /// Current bridge status summary (JSON-friendly).
    pub fn status(&self) -> BusBridgeStatus {
        BusBridgeStatus {
            connected: self.connected.load(Ordering::Relaxed),
            total_received: self.total_received.load(Ordering::Relaxed),
            bus_url: self.bus_url.clone(),
            buffer_capacity: self.capacity,
        }
    }

    /// Buffer size (number of envelopes currently held).
    pub async fn buffer_len(&self) -> usize {
        self.buffer.read().await.len()
    }

    // ── internal ──────────────────────────────────────────────────────

    /// Background loop: connect to SSE, read envelopes, push to buffer.
    /// Reconnects on failure.
    async fn reader_loop(&self) {
        loop {
            info!(url = %self.bus_url, "BusBridge: connecting to bus SSE stream");
            match self.read_sse_stream().await {
                Ok(()) => {
                    info!("BusBridge: SSE stream closed normally");
                }
                Err(e) => {
                    warn!(error = %e, "BusBridge: SSE stream error, reconnecting");
                }
            }
            self.connected.store(false, Ordering::Relaxed);
            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    }

    /// Single SSE connection attempt.  Reads until the stream closes or errors.
    async fn read_sse_stream(&self) -> anyhow::Result<()> {
        let mut req = self
            .client
            .get(&self.bus_url)
            .header("Accept", "text/event-stream");
        if let Some(token) = self
            .auth_token
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await?;

        if !resp.status().is_success() {
            anyhow::bail!("SSE endpoint returned {}", resp.status());
        }

        self.connected.store(true, Ordering::Relaxed);
        info!("BusBridge: connected to SSE stream");

        // Read line-by-line.  SSE format:
        //   event: <type>\n
        //   data: <json>\n
        //   \n
        let mut event_type = String::new();
        let mut data_buf = String::new();

        use futures::StreamExt;
        let mut byte_stream = resp.bytes_stream();

        // Accumulate raw bytes into lines
        let mut line_buf = String::new();

        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);

            for ch in text.chars() {
                if ch == '\n' {
                    let line = std::mem::take(&mut line_buf);
                    self.process_sse_line(&line, &mut event_type, &mut data_buf)
                        .await;
                } else {
                    line_buf.push(ch);
                }
            }
        }

        Ok(())
    }

    /// Process a single SSE text line.
    async fn process_sse_line(&self, line: &str, event_type: &mut String, data_buf: &mut String) {
        if line.is_empty() {
            // Empty line = end of event
            if event_type == "bus" && !data_buf.is_empty() {
                match serde_json::from_str::<BusEnvelope>(data_buf) {
                    Ok(envelope) => {
                        self.push_envelope(envelope).await;
                    }
                    Err(e) => {
                        debug!(error = %e, "BusBridge: failed to parse bus envelope");
                    }
                }
            }
            event_type.clear();
            data_buf.clear();
        } else if let Some(rest) = line.strip_prefix("event:") {
            *event_type = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("data:") {
            if !data_buf.is_empty() {
                data_buf.push('\n');
            }
            data_buf.push_str(rest.trim());
        }
        // Ignore comment lines (`:`) and other prefixes
    }

    /// Push an envelope into the ring buffer, evicting oldest if full.
    async fn push_envelope(&self, envelope: BusEnvelope) {
        let mut buf = self.buffer.write().await;
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(envelope);
        drop(buf);
        self.total_received.fetch_add(1, Ordering::Relaxed);
    }
}

/// Resolve a worker's first-class bus connection via the control plane.
pub async fn resolve_worker_bus_url(
    control_plane_url: &str,
    worker_id: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let worker_url = format!(
        "{}/v1/agent/workers/{}",
        control_plane_url.trim_end_matches('/'),
        worker_id
    );

    let client = reqwest::Client::new();
    let mut req = client.get(worker_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let worker: serde_json::Value = req.send().await?.error_for_status()?.json().await?;

    let has_bus_interface = worker
        .get("interfaces")
        .and_then(|value| value.get("bus"))
        .and_then(|value| value.get("stream_url"))
        .and_then(|value| value.as_str())
        .is_some();
    let has_http_interface = worker
        .get("interfaces")
        .and_then(|value| value.get("http"))
        .and_then(|value| value.get("base_url"))
        .and_then(|value| value.as_str())
        .is_some();

    if has_bus_interface || has_http_interface {
        return Ok(format!(
            "{}/v1/agent/workers/{}/bus/stream",
            control_plane_url.trim_end_matches('/'),
            worker_id
        ));
    }

    anyhow::bail!(
        "Worker '{}' does not advertise a first-class bus interface",
        worker_id
    )
}

#[derive(Debug, Deserialize)]
struct WorkspaceSummary {
    id: String,
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceDetails {
    worker_id: Option<String>,
}

pub async fn resolve_worker_bus_url_for_workspace(
    control_plane_url: &str,
    workspace_id: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let workspace_url = format!(
        "{}/v1/agent/workspaces/{}",
        control_plane_url.trim_end_matches('/'),
        urlencoding::encode(workspace_id)
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workspace_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workspace: WorkspaceDetails = req.send().await?.error_for_status()?.json().await?;
    let worker_id = workspace
        .worker_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Workspace '{}' is not currently assigned to a worker; pass --worker-id or register the workspace on a worker",
                workspace_id
            )
        })?;

    resolve_worker_bus_url(control_plane_url, worker_id, token).await
}

pub async fn resolve_workspace_id_from_path(
    control_plane_url: &str,
    workspace_root: &Path,
    token: Option<&str>,
) -> anyhow::Result<Option<String>> {
    let workspace_root = normalize_local_path(workspace_root)?;
    let workspace_root = workspace_root.to_string_lossy().to_string();
    let workspaces_url = format!(
        "{}/v1/agent/workspaces",
        control_plane_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workspaces_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workspaces: Vec<WorkspaceSummary> = req.send().await?.error_for_status()?.json().await?;
    Ok(best_workspace_match(&workspace_root, &workspaces).map(|workspace| workspace.id.clone()))
}

/// Resolve the default worker bus connection when the control plane has exactly
/// one active worker advertising a first-class bus/http interface.
pub async fn resolve_default_worker_bus_url(
    control_plane_url: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let workers_url = format!(
        "{}/v1/agent/workers",
        control_plane_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workers_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workers: Vec<serde_json::Value> = req.send().await?.error_for_status()?.json().await?;

    let candidates = workers
        .into_iter()
        .filter(|worker| {
            let status = worker
                .get("status")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            status == "active"
                && worker
                    .get("interfaces")
                    .and_then(|value| value.as_object())
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    match candidates.as_slice() {
        [worker] => {
            let worker_id = worker
                .get("worker_id")
                .and_then(|value| value.as_str())
                .ok_or_else(|| anyhow::anyhow!("Active worker is missing worker_id"))?;
            resolve_worker_bus_url(control_plane_url, worker_id, token).await
        }
        [] => anyhow::bail!(
            "No active workers with first-class interfaces were found; deploy/register a worker or provide --worker-id"
        ),
        workers => {
            let worker_ids = workers
                .iter()
                .filter_map(|worker| worker.get("worker_id").and_then(|value| value.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "Multiple active workers are registered ({worker_ids}); provide --worker-id to choose one"
            )
        }
    }
}

/// Status snapshot returned by [`BusBridge::status`].
#[derive(Debug, Clone, serde::Serialize)]
pub struct BusBridgeStatus {
    pub connected: bool,
    pub total_received: u64,
    pub bus_url: String,
    pub buffer_capacity: usize,
}

// ─── Helpers ─────────────────────────────────────────────────────────────

/// Topic pattern matching (mirrors `server/mod.rs::topic_matches`).
fn topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    if let Some(suffix) = pattern.strip_prefix(".*") {
        return topic.ends_with(suffix);
    }
    topic == pattern
}

fn normalize_local_path(path: &Path) -> anyhow::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    Ok(std::env::current_dir()?.join(path))
}

fn best_workspace_match<'a>(
    workspace_root: &str,
    workspaces: &'a [WorkspaceSummary],
) -> Option<&'a WorkspaceSummary> {
    let direct = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            if workspace_root == path || workspace_root.starts_with(&format!("{}/", path)) {
                Some((path.len(), workspace))
            } else {
                None
            }
        })
        .max_by_key(|(path_len, _)| *path_len)
        .map(|(_, workspace)| workspace);

    if direct.is_some() {
        return direct;
    }

    let mut scored: Vec<(usize, &WorkspaceSummary)> = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            let score = shared_path_suffix_score(workspace_root, path);
            (score > 0).then_some((score, workspace))
        })
        .collect();

    scored.sort_by(|left, right| right.0.cmp(&left.0));

    match scored.as_slice() {
        [] => None,
        [(score, workspace), ..] => {
            let is_unique_best = scored
                .get(1)
                .map(|(next_score, _)| next_score < score)
                .unwrap_or(true);
            if is_unique_best {
                Some(*workspace)
            } else {
                None
            }
        }
    }
}

fn shared_path_suffix_score(left: &str, right: &str) -> usize {
    let left_parts: Vec<&str> = left.split('/').filter(|part| !part.is_empty()).collect();
    let right_parts: Vec<&str> = right.split('/').filter(|part| !part.is_empty()).collect();

    let mut score = 0usize;
    for (left_part, right_part) in left_parts.iter().rev().zip(right_parts.iter().rev()) {
        if left_part == right_part {
            score += 1;
        } else {
            break;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_matches() {
        assert!(topic_matches("agent.123.events", "*"));
        assert!(topic_matches("agent.123.events", "agent.*"));
        assert!(topic_matches("ralph.prd1", "ralph.*"));
        assert!(!topic_matches("task.42", "agent.*"));
        assert!(topic_matches("agent.123.events", "agent.123.events"));
    }
}
