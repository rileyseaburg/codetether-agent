//! HTTP client for A2A discovery, messages and task operations.
//!
//! [A2AClient] wraps the shared wire contracts. Construct a client for a peer
//! URL and optionally use [A2AClient::with_token] before issuing requests.
use anyhow::Result;
use codetether_a2a_core::*;
use reqwest::Client;
mod construct;
mod discovery;
mod message;
mod rpc;
mod tasks;

/// A2A Client for interacting with A2A agents
pub struct A2AClient {
    client: Client,
    base_url: String,
    auth: Option<AuthenticationInfo>,
}

/// Request readers, task records and table transitions for HTTP servers.
pub mod task_state;
