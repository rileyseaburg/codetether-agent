//! Routes and stable identity aliases learned through peer discovery.
//!
//! [PeerRoute] stores endpoint metadata. Registering a card replaces its stale
//! aliases; lookup supports either the card name or stable provenance identity.
use codetether_a2a_core::AgentCard;
use dashmap::DashMap;
use std::sync::LazyLock;
mod aliases;
mod operations;
mod record;
pub use operations::{get, list, register, remove};
pub use record::PeerRoute;

static ROUTES: LazyLock<DashMap<String, PeerRoute>> = LazyLock::new(DashMap::new);
