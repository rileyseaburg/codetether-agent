//! HTTP agent-card probing for configured and locally discovered peers.
//!
//! [try_fetch_agent_card] performs one authenticated card request. [resolve]
//! applies readiness retries to a discovery record and skips the local URL.
//! The caller retains peer registration and introduction responsibilities.
//!
//! ```rust,no_run
//! # async fn example() -> anyhow::Result<()> {
//! use codetether_a2a_http::peer_probe::try_fetch_agent_card;
//! let card = try_fetch_agent_card("http://localhost:4096").await?;
//! assert!(!card.url.is_empty());
//! # Ok(()) }
//! ```
use codetether_a2a_core::AgentCard;
use codetether_a2a_discovery::{endpoint::peer_candidates, mdns::DiscoveredPeer};
use std::time::Duration;
mod fetch;
mod resolve;
mod resolve_once;
pub use fetch::try_fetch_agent_card;
pub use resolve::resolve;
use resolve_once::resolve_once;
const ATTEMPTS: u32 = 5;
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);
