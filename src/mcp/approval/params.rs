use super::JsonRpcError;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(super) struct RequestParams {
    pub tool: String,
    pub action: String,
    pub resource: String,
    pub reason: Option<String>,
    pub tool_call_id: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct DecisionParams {
    pub approval_id: String,
    pub decision: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
}

pub(super) fn parse<T: for<'de> Deserialize<'de>>(
    params: Option<Value>,
) -> Result<T, JsonRpcError> {
    let value = params.ok_or_else(|| JsonRpcError::invalid_params("Missing params"))?;
    serde_json::from_value(value).map_err(|error| JsonRpcError::invalid_params(error.to_string()))
}
