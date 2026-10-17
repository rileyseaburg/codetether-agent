//! stream wire schema.
use super::*;
use serde::{Deserialize, Serialize};
// ============== Streaming Types ==============

#[derive(Debug, Deserialize)]
pub(crate) struct StreamChunkResponse {
    pub(crate) choices: Vec<StreamChoice>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct StreamChoice {
    pub(crate) delta: StreamDelta,
    pub(crate) finish_reason: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct StreamDelta {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<StreamToolCall>>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct StreamToolCall {
    #[allow(dead_code)]
    pub(crate) index: usize,
    #[serde(default)]
    pub(crate) id: Option<String>,
    #[serde(default)]
    pub(crate) function: Option<StreamToolFunction>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct StreamToolFunction {
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) arguments: Option<String>,
}
