//! Shared GitHub PR context, lifecycle policy, and body rendering.
//!
//! [context] reads repository metadata, [body] renders execution signatures,
//! [provenance_block] renders signed provenance, and [autopilot] plus [ci_watcher]
//! interpret lifecycle and CI state. Credential acquisition and CLI I/O belong
//! to the embedding application.
pub mod autopilot;
pub mod body;
pub mod ci_watcher;
pub mod context;
pub mod provenance_block;

/// PR HTTP operations using caller-provided credentials.
pub mod http;
