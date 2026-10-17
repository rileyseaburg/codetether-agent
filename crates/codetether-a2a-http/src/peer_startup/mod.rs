//! Listener preparation and discovery startup for A2A peers.
//!
//! [SpawnOptions] selects the endpoint; [StartupHooks] supplies application
//! bindings. [prepare_a2a] returns an [A2APreparation] ready to serve.
use crate::peer_intake::{discovery_loop, mdns_intake_loop};
use anyhow::{Context, Result};
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_a2a_discovery::{collaboration_token, lan, mdns, mdns::DiscoveredPeer};
use codetether_a2a_discovery::{
    endpoint::{collect_peers, concrete, first_lan_ipv4, normalize_base_url},
    naming::auto_agent_name,
};
use codetether_bus_core::AgentBus;
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::{sync::mpsc, task::JoinHandle};
mod bind;
mod hooks;
mod local;
mod options;
mod prepare;
mod record;
mod seeds;
use bind::Binding;
pub use hooks::StartupHooks;
pub use options::SpawnOptions;
pub use prepare::prepare_a2a;
pub use record::A2APreparation;

mod background;
mod foreground;
/// Start background serving of a prepared endpoint.
pub use background::start_background;
/// Serve a prepared endpoint until the supplied shutdown signal.
pub use foreground::serve_foreground;
