//! Record non-empty assistant text in the context trace.
use super::*;
impl RlmExecutor {
    pub(super) fn trace_analysis_response(
        &mut self,
        response: &codetether_provider_core::CompletionResponse,
    ) {
        let assistant_text = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        if !assistant_text.is_empty() {
            self.context_trace.log_event(ContextEvent::AssistantCode {
                code: truncate_with_ellipsis(&assistant_text, 500),
                tokens: ContextTrace::estimate_tokens(&assistant_text),
            });
        }
    }
}
