//! A2A Protocol Implementation
//!
//! First-class support for the Agent-to-Agent (A2A) protocol, enabling
//! this agent to work as both a client and server in the A2A ecosystem.
//!
//! ## Transports
//! - **JSON-RPC** (default) — native server adapter and shared HTTP client
//! - **gRPC** — shared gRPC service (tonic-based, compiled from `crates/codetether-a2a-grpc/proto/a2a/v1/a2a.proto`)

pub use codetether_a2a_discovery::address as mdns_addr;
pub use codetether_a2a_discovery::lan;
pub use codetether_a2a_discovery::liveness as peer_liveness;
pub use codetether_a2a_discovery::presence as mdns_liveness;
pub use codetether_a2a_grpc::bridge;
pub use codetether_a2a_grpc::server as grpc;
pub use codetether_a2a_http as client;
pub(crate) use codetether_a2a_http::identity as agent_identity;
pub use codetether_a2a_inbox as live_inbox;
pub use codetether_a2a_worker_core::claim;
pub use codetether_bus_core::peer_messages as bus_emit;
pub use codetether_native_a2a::git_credentials;
pub use codetether_native_a2a::intro;
pub use codetether_native_a2a::mdns;
pub(crate) mod peer_route;
pub use codetether_native_a2a::prompt_runtime;
pub mod server;
pub use codetether_a2a_http::auth as server_auth;
pub use codetether_native_a2a::session_config;
pub mod session_resolve;
pub mod spawn;
pub use codetether_a2a_core as types;
pub use codetether_a2a_grpc::voice as voice_grpc;
pub use codetether_a2a_worker_core::task_scope;
pub use codetether_native_a2a::stream;
pub mod worker;
pub mod worker_tool_registry;
pub use codetether_a2a_http::workspace as worker_workspace_context;
pub use codetether_a2a_http::workspace as worker_workspace_record;

/// Generated protobuf/tonic types from `crates/codetether-a2a-grpc/proto/a2a/v1/a2a.proto`.
pub use codetether_a2a_grpc::proto;

// Re-export commonly used types
#[allow(unused_imports)]
pub use client::A2AClient;
