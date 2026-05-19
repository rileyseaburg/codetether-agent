use super::{ContextEvent, ContextTrace};

impl ContextTrace {
    pub(super) fn grep_result(pattern: String, matches: usize, tokens: usize) -> ContextEvent {
        ContextEvent::GrepResult {
            pattern,
            matches,
            tokens,
        }
    }

    pub(super) fn llm_result(
        query: String,
        response_preview: String,
        tokens: usize,
    ) -> ContextEvent {
        ContextEvent::LlmQueryResult {
            query,
            response_preview,
            tokens,
        }
    }

    pub(super) fn tool_call(
        name: String,
        arguments_preview: String,
        tokens: usize,
    ) -> ContextEvent {
        ContextEvent::ToolCall {
            name,
            arguments_preview,
            tokens,
        }
    }

    pub(super) fn tool_result(
        tool_call_id: String,
        result_preview: String,
        tokens: usize,
    ) -> ContextEvent {
        ContextEvent::ToolResult {
            tool_call_id,
            result_preview,
            tokens,
        }
    }
}
