//! A2A Protocol Implementation
//!
//! First-class support for the Agent-to-Agent (A2A) protocol, enabling
//! this agent to work as both a client and server in the A2A ecosystem.
//!
//! ## Transports
//! - **JSON-RPC** (default) — `server.rs` / `client.rs`
//! - **gRPC** — `grpc.rs` (tonic-based, compiled from `crates/codetether-a2a-grpc/proto/a2a/v1/a2a.proto`)

pub use codetether_a2a_grpc::bridge;
pub(crate) use codetether_a2a_http::identity as agent_identity;
pub mod bus_emit;
pub mod claim;
pub mod client;
pub(crate) mod collaboration_token;
#[path = "git_credentials/mod.rs"]
pub mod git_credentials;
pub mod grpc;
pub mod intro;
pub mod lan;
pub use codetether_a2a_inbox as live_inbox;
pub(crate) mod local_identity;
pub mod mdns;
pub use codetether_a2a_discovery::address as mdns_addr;
pub use codetether_a2a_discovery::presence as mdns_liveness;
pub mod peer_liveness;
pub(crate) mod peer_prompt;
pub(crate) mod peer_route;
pub mod prompt_runtime;
pub mod server;
pub mod server_auth;
pub mod session_config;
pub mod session_resolve;
pub mod spawn;
pub mod stream;
pub mod task_scope;
pub use codetether_a2a_core as types;
pub mod voice_grpc;
pub mod worker;
pub mod worker_tool_registry;
pub mod worker_workspace_context;
pub mod worker_workspace_record;

/// Generated protobuf/tonic types from `crates/codetether-a2a-grpc/proto/a2a/v1/a2a.proto`.
pub use codetether_a2a_grpc::proto;

// Re-export commonly used types
#[allow(unused_imports)]
pub use client::A2AClient;
