//! Injected-client history conversion using its existing conventions.
use crate::Message;
use serde_json::Value;
pub(crate) fn convert(messages: &[Message], include_reasoning: bool) -> Vec<Value> {
    crate::history::convert(
        messages,
        include_reasoning,
        crate::history::HistoryFormat::ExpandedReplies,
        crate::tool_arguments::normalize,
    )
}
