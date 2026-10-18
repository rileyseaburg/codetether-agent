//! Validate tool invocation parameters before publishing or executing a call.
use codetether_mcp_core::types::{CallToolParams, JsonRpcError};
use serde_json::Value;

pub(super) fn parse(params: Option<Value>) -> Result<CallToolParams, JsonRpcError> {
    let params = params.ok_or_else(|| JsonRpcError::invalid_params("Missing params"))?;
    serde_json::from_value(params).map_err(|error| JsonRpcError::invalid_params(error.to_string()))
}
