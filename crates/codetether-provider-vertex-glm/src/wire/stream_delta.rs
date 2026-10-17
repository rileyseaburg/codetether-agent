//! Wire representation of StreamDelta.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct StreamDelta {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<StreamToolCall>>,
}
