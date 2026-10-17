//! Wire representation of ChoiceMessage.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ChoiceMessage {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<ToolCall>>,
}
