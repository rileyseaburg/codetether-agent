//! Glm5ToolCall wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5ToolCall {
    pub(crate) id: String,
    pub(crate) function: Glm5Function,
}
