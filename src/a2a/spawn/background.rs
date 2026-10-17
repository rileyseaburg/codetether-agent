//! Peer startup background.
use super::*;

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
    Ok(codetether_a2a_http::peer_startup::start_background(prep))
}
