//! Worker initialization using application-supplied catalog and peer hooks.
//!
//! [initialize] preserves shared worker startup sequencing. [Hooks] supplies
//! the two operations that require the application's complete tool/server types.
use codetether_a2a_http::peer_handle::A2APeerHandle;
use codetether_bus_core::{AgentBus, BusHandle};
use codetether_cli_options::A2aArgs;
use std::{future::Future, sync::Arc};
mod host;
/// Application-specific tool catalog and peer startup operations.
///
/// Implementors delegate to the application's existing catalog and peer runtime.
pub trait Hooks {
    /// Publish the concrete tool schemas, returning the receiver count.
    fn announce(handle: &BusHandle) -> usize;
    /// Start the application peer, retaining its existing optional result.
    fn peer(
        args: &A2aArgs,
        name: &str,
        bus: Arc<AgentBus>,
    ) -> impl Future<Output = Option<A2APeerHandle>> + Send;
}
/// Construct worker state and start services using the supplied hooks.
/// # Arguments
/// * `args` - Original worker options retained in the returned context.
/// # Returns
/// The initialized native worker context.
/// # Errors
/// Returns HTTP-client initialization errors from the shared startup sequence.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_native_worker::initialization::Hooks>(args: codetether_cli_options::A2aArgs) -> anyhow::Result<()> {
/// let _context = codetether_native_worker::initialization::initialize::<H>(args).await?;
/// # Ok(()) }
/// ```
pub async fn initialize<H: Hooks>(args: A2aArgs) -> anyhow::Result<crate::WorkerContext> {
    codetether_a2a_worker_core::initialization::initialize::init_worker::<host::Native<H>>(args)
        .await
}
