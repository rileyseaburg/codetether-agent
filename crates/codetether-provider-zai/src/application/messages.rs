//! Preserve the application history format and normalization.
use super::ZaiProvider;
use crate::Message;
use serde_json::Value;
impl ZaiProvider {
    pub(super) fn normalize_tool_arguments(arguments: &str) -> String {
        crate::tool_arguments::normalize(arguments)
    }
    pub(super) fn convert_messages(
        messages: &[Message],
        include_reasoning_content: bool,
    ) -> Vec<Value> {
        crate::history::convert(
            messages,
            include_reasoning_content,
            crate::history::HistoryFormat::SingleReply,
            Self::normalize_tool_arguments,
        )
    }
}
