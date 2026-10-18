//! Prepare a background peer on an existing application bus.
use crate::{
    peer_handle::A2APeerHandle,
    peer_startup::{SpawnOptions, StartupHooks, prepare_a2a, start_background},
};
use codetether_bus_core::AgentBus;
use std::sync::Arc;
/// Prepare and start a peer on the supplied shared bus.
/// # Arguments
/// `options` selects the endpoint; `bus` and `hooks` retain the application's runtime identity.
/// # Returns
/// The peer handle, which the caller keeps alive while serving is desired.
/// # Errors
/// Propagates peer preparation errors.
/// # Examples
/// ```no_run
/// # async fn example(hooks: codetether_a2a_http::peer_startup::StartupHooks, bus: std::sync::Arc<codetether_bus_core::AgentBus>) -> anyhow::Result<()> {
/// let handle = codetether_a2a_http::peer_startup::application::background(
///     codetether_a2a_http::peer_startup::SpawnOptions::auto(), bus, hooks,
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn background(
    options: SpawnOptions,
    bus: Arc<AgentBus>,
    hooks: StartupHooks,
) -> anyhow::Result<A2APeerHandle> {
    let prep = prepare_a2a(options, bus, hooks).await?;
    Ok(start_background(prep))
}
