//! ConverseContent wire shape.
use super::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ConverseContent {
    ReasoningContent {
        #[serde(rename = "reasoningContent")]
        reasoning_content: ReasoningContentBlock,
    },
    Text {
        text: String,
    },
    ToolUse {
        #[serde(rename = "toolUse")]
        tool_use: ConverseToolUse,
    },
}
