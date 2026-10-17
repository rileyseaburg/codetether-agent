//! Peer discovery primitives for A2A endpoints.
//!
//! [mdns] advertises and resolves local services, [address] ranks candidate
//! endpoints, and [liveness::PeerLiveness] tracks duplicate and expired sightings.
//! Applications supply their advertised version and own bus/registry updates.
//!
//! ```rust
//! use codetether_a2a_discovery::liveness::PeerLiveness;
//! let mut peers = PeerLiveness::new();
//! assert!(peers.record("instance", "agent"));
//! assert!(!peers.record("instance", "agent"));
//! ```
pub mod address;
pub mod liveness;
pub mod mdns;

/// Learned peer routes and stable identity aliases.
pub mod routes;

/// UDP broadcast fallback for local peer discovery.
pub mod lan;

/// Configured peer URLs and advertised interface selection.
pub mod endpoint;

/// Process-local capability advertised by peer discovery.
pub mod collaboration_token;

/// Active local peer identity and lifecycle tracking.
pub mod local_identity;

/// Live collaborator context for model prompts.
pub mod peer_prompt;

/// Automatic local peer names.
pub mod naming;

/// Workspace-aware peer descriptions.
pub mod description;
