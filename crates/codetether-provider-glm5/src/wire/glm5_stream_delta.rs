//! Glm5StreamDelta wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5StreamDelta {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<Glm5StreamToolCall>>,
}
