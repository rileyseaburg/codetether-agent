//! MCP approval request/decision JSON-RPC surface.
//!
//! [capability] advertises the supported methods. [handle] parses requests,
//! persists decisions through the shared approval store, applies session grants
//! and delivers decisions to existing live waiters.

mod capability;
mod decision;
mod kind;
mod params;
mod request;
mod response;

use codetether_mcp_core::types::JsonRpcError;
use serde_json::Value;

/// Advertise the MCP approval request and decision methods.
/// # Returns
/// Experimental approval capability metadata.
/// # Examples
/// ```
/// assert!(codetether_mcp_approval::capability().is_some());
/// ```
pub fn capability() -> Option<Value> {
    capability::value()
}

/// Handle an approval request or decision method.
/// # Arguments
/// * `method` - Approval JSON-RPC method, including its accepted aliases.
/// * `params` - Request or decision parameters.
/// # Returns
/// A pending-request or recorded-decision response.
/// # Errors
/// Returns invalid-parameter, store or unknown-method JSON-RPC errors.
/// # Examples
/// ```no_run
/// async fn missing() {
///     assert!(codetether_mcp_approval::handle("unknown", None).await.is_err());
/// }
/// ```
pub async fn handle(method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
    match method {
        "approvals/request" | "approval/request" => request::handle(params),
        "approvals/decision" | "approvals/decide" | "approval/decision" | "approval/decide" => {
            decision::handle(params)
        }
        _ => Err(JsonRpcError::method_not_found(method)),
    }
}

/// Outbound MCP tool invocation policy and local metadata stripping.
pub mod outbound;
