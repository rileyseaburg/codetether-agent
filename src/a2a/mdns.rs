//! mDNS discovery for peer-to-peer A2A services.
//!
//! This module advertises the local A2A endpoint and emits other resolved
//! endpoints as [`DiscoveredPeer`] values. [`MdnsHandle`] owns the registration
//! lifecycle, while [`announce_and_browse`] starts advertisement and discovery.
//!
//! # Usage
//!
//! ```rust,no_run
//! use codetether_agent::a2a::mdns::announce_and_browse;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! let (peer_tx, _peer_rx) = tokio::sync::mpsc::channel(8);
//! let address = IpAddr::V4(Ipv4Addr::LOCALHOST);
//! let handle = announce_and_browse("local-agent", 8000, vec![address], "token", peer_tx)?;
//! handle.shutdown();
//! # Ok::<(), anyhow::Error>(())
//! ```

#[path = "mdns_announcement.rs"]
mod announcement;
#[path = "mdns_browser.rs"]
mod browser;
#[path = "mdns_event.rs"]
mod event;
#[path = "mdns_handle.rs"]
mod handle;
#[path = "mdns_hostname.rs"]
mod hostname;
#[path = "mdns_peer.rs"]
mod peer;
#[path = "mdns_resolved.rs"]
mod resolved;
#[path = "mdns_resolved_addresses.rs"]
mod resolved_addresses;
#[path = "mdns_scope.rs"]
mod scope;
#[path = "mdns_service.rs"]
mod service;

pub use announcement::announce_and_browse;
pub use handle::MdnsHandle;
pub use hostname::sanitize_hostname;
pub use peer::DiscoveredPeer;

/// DNS-SD service type shared by all CodeTether A2A peers.
pub const SERVICE_TYPE: &str = "_codetether-a2a._tcp.local.";
