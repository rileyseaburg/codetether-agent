//! Concurrent HTTP resolution of peers found by local discovery.
//!
//! [mdns_intake_loop] refreshes peer presence, registers routes, and optionally
//! introduces the local agent. Callers supply the bus and ledger-path resolver.
use crate::{intro::send::send_intro, peer_probe as mdns_probe};
use codetether_a2a_discovery::{liveness::PeerLiveness, mdns::DiscoveredPeer, presence, routes};
use codetether_bus_core::AgentBus;
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use tokio::sync::{Mutex, mpsc};
mod accept;
mod run;
use accept::handle_mdns_peer;
pub use run::mdns_intake_loop;
