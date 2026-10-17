//! ConverseToolUse wire shape.
use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConverseToolUse {
    pub(crate) tool_use_id: String,
    pub(crate) name: String,
    pub(crate) input: Value,
}
