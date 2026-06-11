//! MCP approval request/decision JSON-RPC surface.

mod capability;
mod decision;
mod kind;
mod params;
mod request;
mod response;

use super::types::JsonRpcError;
use serde_json::Value;

pub(super) fn capability() -> Option<Value> {
    capability::value()
}

pub(super) async fn handle(method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
    match method {
        "approvals/request" | "approval/request" => request::handle(params),
        "approvals/decision" | "approvals/decide" | "approval/decision" | "approval/decide" => {
            decision::handle(params)
        }
        _ => Err(JsonRpcError::method_not_found(method)),
    }
}

#[cfg(test)]
#[path = "approval/session_decision_tests.rs"]
mod session_decision_tests;
#[cfg(test)]
mod tests;
