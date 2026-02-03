//! A2A Protocol Implementation
//!
//! First-class support for the Agent-to-Agent (A2A) protocol, enabling
//! this agent to work as both a client and server in the A2A ecosystem.

pub mod client;
pub mod server;
pub mod types;
pub mod worker;

// Re-export commonly used types
#[allow(unused_imports)]
pub use client::A2AClient;

