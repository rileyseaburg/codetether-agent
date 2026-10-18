//! Optional TUI peer startup, readiness and retained endpoint ownership.
//!
//! Call [start] with application startup and inbox hooks. [PeerEndpoint]
//! keeps background serving alive until the TUI runtime drops its handle.
use std::{future::Future, sync::Arc};

use codetether_a2a_http::{peer_handle::A2APeerHandle, peer_startup::SpawnOptions};
use codetether_bus_core::AgentBus;

/// Retains the optional peer endpoint for the lifetime of the TUI runtime.
pub struct PeerEndpoint {
    /// Whether startup established a usable peer endpoint.
    pub ready: bool,
    _handle: Option<A2APeerHandle>,
}

/// Start an optional peer while retaining startup's best-effort behavior.
/// # Arguments
/// Receives optional peer settings, the live bus, the application's endpoint
/// starter and the inbox hook to invoke after a successful start.
/// # Returns
/// A retained ready endpoint on success, or an inactive endpoint when disabled
/// or when startup fails. Failures are logged and do not abort the TUI.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let bus = codetether_bus_core::AgentBus::new().into_arc();
/// let peer = codetether_tui_startup::peer::start(None, bus,
///     |_, _| async { anyhow::bail!("unused") }, || {}).await;
/// assert!(!peer.ready);
/// # }
/// ```
pub async fn start<F>(
    options: Option<SpawnOptions>,
    bus: Arc<AgentBus>,
    start_peer: impl FnOnce(SpawnOptions, Arc<AgentBus>) -> F,
    attach: impl FnOnce(),
) -> PeerEndpoint
where
    F: Future<Output = anyhow::Result<A2APeerHandle>>,
{
    let Some(options) = options else {
        return PeerEndpoint {
            ready: false,
            _handle: None,
        };
    };
    match start_peer(options, bus).await {
        Ok(handle) => {
            // Peer turns run in the visible session by default; `--no-a2a`
            // skips this whole endpoint, `/a2a headless` opts out at runtime.
            attach();
            tracing::info!(target: "codetether_agent::tui::app::run::peer",
                agent = %handle.agent_name,
                bind_addr = %handle.bind_addr,
                public_url = %handle.public_url,
                "TUI A2A peer endpoint ready"
            );
            PeerEndpoint {
                ready: true,
                _handle: Some(handle),
            }
        }
        Err(err) => {
            tracing::error!(target: "codetether_agent::tui::app::run::peer",error = %err, "Failed to start TUI A2A peer; continuing without it");
            PeerEndpoint {
                ready: false,
                _handle: None,
            }
        }
    }
}
