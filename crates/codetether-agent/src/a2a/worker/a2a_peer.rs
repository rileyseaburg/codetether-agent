//! Bind shared worker peer startup to the application's bus and endpoint.
use crate::{a2a::spawn::A2APeerHandle, bus::AgentBus, cli::A2aArgs};
use std::sync::Arc;
pub(super) async fn maybe_start_worker_a2a_peer(
    args: &A2aArgs,
    name: &str,
    bus: Arc<AgentBus>,
) -> Option<A2APeerHandle> {
    codetether_a2a_http::peer_startup::worker::maybe_start_worker_a2a_peer(
        &args.hostname,
        args.public_url.as_deref(),
        name,
        |options| crate::a2a::spawn::start_a2a_in_background(options, bus),
    )
    .await
}
