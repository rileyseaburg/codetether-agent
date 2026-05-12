//! Auto-discovery of an already-running Chromium browser that has the
//! DevTools Protocol exposed on a local port.
//!
//! Chrome / Edge / Brave only allow one process per user-data-dir, so every
//! time the agent calls `start` with the same profile directory a fresh
//! spawn races the still-running one and dies with "unexpected end of
//! stream". The fix is to notice the existing instance and attach to it.
//!
//! The user enables this by launching their browser once with
//! `--remote-debugging-port=9222`. From then on every `start` call reuses
//! the same window — preserving tabs, cookies, logged-in sessions, and
//! whatever the human has already navigated to.

use std::time::Duration;

/// Ports we probe, in priority order.
///
/// - `9222` is the documented Chrome default and what every tutorial tells
///   users to pass.
/// - `9223` is the common secondary (e.g. Edge + Chrome side by side).
/// - `9224`/`9225` catch users running multiple debuggable profiles.
///
/// The `CODETETHER_BROWSER_DEBUG_PORTS` env var (comma-separated list) can
/// override this when the user has a site-wide convention.
const DEFAULT_PROBE_PORTS: &[u16] = &[9222, 9223, 9224, 9225];

fn probe_ports() -> Vec<u16> {
    if let Ok(raw) = std::env::var("CODETETHER_BROWSER_DEBUG_PORTS") {
        let parsed: Vec<u16> = raw
            .split(',')
            .filter_map(|s| s.trim().parse::<u16>().ok())
            .collect();
        if !parsed.is_empty() {
            return parsed;
        }
    }
    DEFAULT_PROBE_PORTS.to_vec()
}

/// Return the first port from [`PROBE_PORTS`] that can be bound — i.e. that
/// no other process is listening on. This lets the launcher pin Chrome's
/// DevTools endpoint to a predictable port so the **next** `start` call
/// auto-attaches to the already-running browser.
///
/// Returns `None` if all candidate ports are in use; callers treat that as
/// "let chromiumoxide pick an ephemeral port" — the session works, but the
/// reconnect-on-next-turn benefit is lost until the human frees 9222/9223.
pub(super) fn pick_free_port() -> Option<u16> {
    use std::net::TcpListener;
    for port in probe_ports() {
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return Some(port);
        }
    }
    None
}

/// Return the `webSocketDebuggerUrl` of an already-running browser, if any.
///
/// Returns `None` (never an error) so the caller can fall through to
/// launching. A 500 ms per-port budget keeps the common "no browser" case
/// from blocking `start` for long.
pub(super) async fn detect_running_browser() -> Option<String> {
    for port in probe_ports() {
        if let Some(ws) = resolve_ws_url(&format!("http://127.0.0.1:{port}")).await {
            tracing::info!(%ws, port, "attaching to running browser with remote debugging");
            return Some(ws);
        }
    }
    None
}

/// Resolve an `http://host:port` base URL to the actual WebSocket debugger
/// URL by hitting `/json/version`. Returns `None` on any failure — the
/// caller treats "no browser" and "wrong URL" identically.
pub(super) async fn resolve_ws_url(http_base: &str) -> Option<String> {
    let url = format!("{}/json/version", http_base.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .ok()?;
    let response = client.get(&url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body: serde_json::Value = response.json().await.ok()?;
    body.get("webSocketDebuggerUrl")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}
