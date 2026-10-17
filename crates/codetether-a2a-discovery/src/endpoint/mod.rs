//! Peer endpoint normalization and advertised address selection.
//!
//! Use [normalize_base_url] and [collect_peers] for configured seeds,
//! [peer_candidates] for probe ordering, and [first_lan_ipv4] with [concrete]
//! to choose the local address published by discovery. These operations do
//! not start a listener, register peers, or require application runtime state.
//!
//! ```rust
//! use codetether_a2a_discovery::endpoint::normalize_base_url;
//! assert_eq!(normalize_base_url("localhost:4096").unwrap(), "http://localhost:4096");
//! ```
mod lan_address;
mod mdns_addresses;
mod peer_url;

pub use lan_address::first_lan_ipv4;
pub use mdns_addresses::concrete;
pub use peer_url::{collect_peers, normalize_base_url, peer_candidates};
