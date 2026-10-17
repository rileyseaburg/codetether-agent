//! message wire representation.
use serde::Deserialize;
use super::*;
#[derive(Debug, Deserialize)]
pub(crate) struct DsMessage {
    #[serde(default)]
    #[allow(dead_code)]
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<DsToolCall>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsToolCall {
    pub id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub call_type: String,
    pub function: DsFunction,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsFunction {
    pub name: String,
    pub arguments: String,
}
