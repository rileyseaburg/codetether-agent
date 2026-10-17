//! Record initial system-message and query token estimates.
use super::*;
impl RlmExecutor {
    pub(super) fn trace_analysis_initial(&mut self, messages: &[Message], query: &str) {
        let initial_system = messages[0]
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        self.context_trace.log_event(ContextEvent::SystemPrompt {
            content: "rlm_system_prompt".to_string(),
            tokens: ContextTrace::estimate_tokens(&initial_system),
        });
        self.context_trace.log_event(ContextEvent::ToolCall {
            name: "initial_query".to_string(),
            arguments_preview: truncate_with_ellipsis(query, 160),
            tokens: ContextTrace::estimate_tokens(query),
        });
    }
}
