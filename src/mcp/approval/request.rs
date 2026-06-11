use crate::approval::ApprovalStore;
use serde_json::Value;

use super::{JsonRpcError, params, response};

pub(super) fn handle(params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params: params::RequestParams = params::parse(params)?;
    let reason = params
        .reason
        .unwrap_or_else(|| "mcp approval request".to_string());
    let store = ApprovalStore::open_default()
        .map_err(|error| JsonRpcError::internal_error(error.to_string()))?;
    let request = store
        .create_request(&params.tool, &params.action, &params.resource, &reason)
        .map_err(|error| JsonRpcError::internal_error(error.to_string()))?;
    Ok(response::pending(&request, params.tool_call_id))
}
