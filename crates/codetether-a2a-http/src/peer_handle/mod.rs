//! Lifecycle ownership for background A2A HTTP endpoints.
//!
//! [A2APeerHandle] owns server and discovery tasks plus the mDNS advertisement.
//! Construct it from a running endpoint and keep it alive until shutdown.
use codetether_a2a_discovery::{local_identity, mdns};
use tokio::task::JoinHandle;
mod construct;
mod lifecycle;
mod record;
pub use record::A2APeerHandle;
