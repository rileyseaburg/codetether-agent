//! A2A Protocol Implementation
//!
//! First-class support for the Agent-to-Agent (A2A) protocol, enabling
//! this agent to work as both a client and server in the A2A ecosystem.
//!
//! ## Transports
//! - **JSON-RPC** (default) — `server.rs` / `client.rs`
//! - **gRPC** — `grpc.rs` (tonic-based, compiled from `proto/a2a/v1/a2a.proto`)

pub mod bridge;
pub mod client;
pub mod grpc;
pub mod server;
pub mod spawn;
pub mod types;
pub mod voice_grpc;
pub mod worker;

/// Generated protobuf/tonic types from `proto/a2a/v1/a2a.proto`.
pub mod proto {
    tonic::include_proto!("a2a.v1");
}

// Re-export commonly used types
#[allow(unused_imports)]
pub use client::A2AClient;
