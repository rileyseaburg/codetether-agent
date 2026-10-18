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
//! 4. Spawn [`discovery_loop`](codetether_a2a_http::peer_intake::discovery_loop) for `--peer` seeds.
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

use crate::a2a::server::A2AServer;
use crate::bus::AgentBus;
use crate::cli::SpawnArgs;
use anyhow::Result;
use std::sync::Arc;

#[path = "spawn/agent_card.rs"]
mod agent_card;
pub use codetether_a2a_http::peer_handle::A2APeerHandle;
mod background;
mod command;
pub use background::start_a2a_in_background;
pub use codetether_a2a_discovery::naming::auto_agent_name;
use codetether_a2a_http::peer_startup::A2APreparation;
pub use codetether_a2a_http::peer_startup::SpawnOptions;
pub use command::run;
/// Bind application card/router configuration to shared peer startup.
fn startup_hooks() -> codetether_a2a_http::peer_startup::StartupHooks {
    codetether_a2a_http::peer_startup::StartupHooks {
        build_card: agent_card::build,
        build_router: |card, bus| A2AServer::with_bus(card, bus).router(),
        ledger_path: crate::a2a::intro::ledger::ledger_path,
        application_version: env!("CARGO_PKG_VERSION"),
    }
}
