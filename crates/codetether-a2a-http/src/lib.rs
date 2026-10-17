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

/// HTTP routing over host-provided task state and execution hooks.
pub mod server;

/// Workspace metadata and clone-directory resolution for worker tasks.
pub mod workspace;

/// Worker HTTP client and response handling.
pub mod worker;

/// Introduction protocol helpers and bounded delivery.
pub mod intro;

/// Git credential broker protocol and HTTP client.
pub mod git_credentials;

/// Candidate card requests and readiness retries for discovered peers.
pub mod peer_probe;

/// A2A peer bearer-token middleware.
pub mod auth;

/// Concurrent discovery intake, HTTP resolution and peer registration.
pub mod peer_intake;

/// Task ownership and shutdown for background peer endpoints.
pub mod peer_handle;

/// Listener preparation and ordered peer discovery startup.
pub mod peer_startup;

/// Inbound interactive handoff and headless turn selection.
pub mod inbound_turn;

/// Shared message admission over application execution hooks.
pub mod admission;

/// Context-id validation and persistent conversation resolution.
pub mod conversation;
