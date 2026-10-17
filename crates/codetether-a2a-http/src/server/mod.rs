//! A2A HTTP routes independent of session execution.
//!
//! Implement [ServerHost] to supply application state and message execution.
//! [routes] returns a router awaiting host state; callers attach middleware.
use async_trait::async_trait;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use codetether_a2a_core::{AgentCard, JsonRpcError, JsonRpcRequest, JsonRpcResponse, Task};
use dashmap::DashMap;
use serde_json::Value;
mod card;
mod routes;
mod rpc;
pub use routes::routes;

/// Application state and execution hooks consumed by the shared A2A routes.
///
/// Implementors expose their existing card and task table and delegate message
/// execution to their runtime. Both callbacks return JSON-RPC results/errors.
#[async_trait]
pub trait ServerHost: Clone + Send + Sync + 'static {
    /// Card returned by both well-known discovery paths.
    fn agent_card(&self) -> &AgentCard;
    /// Task table used by query and cancellation handlers.
    fn tasks(&self) -> &DashMap<String, Task>;
    /// Execute a message/send request.
    ///
    /// # Errors
    /// Return a JSON-RPC error when parsing or execution fails.
    async fn send_message(&self, request: JsonRpcRequest) -> Result<Value, JsonRpcError>;
    /// Submit a message/stream request using the host's execution policy.
    ///
    /// # Errors
    /// Return a JSON-RPC error when parsing or submission fails.
    async fn stream_message(&self, request: JsonRpcRequest) -> Result<Value, JsonRpcError>;
}
