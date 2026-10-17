//! Record a rejected final tool response without ending the loop.
use super::*;
impl RlmExecutor {
    pub(super) fn reject_structured_final(
        &mut self,
        call: &(String, String, String),
        iterations: usize,
        tool_results: &mut Vec<ContentPart>,
    ) {
        let call_id = &call.0;
        let rejection = "FINAL rejected: pattern query requires grep evidence. Call rlm_grep first, then FINAL with exact line-numbered matches.";
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: "reject_final(no_grep_evidence)".to_string(),
            output: rejection.to_string(),
        });
        self.context_trace.log_event(ContextEvent::ToolResult {
            tool_call_id: call_id.clone(),
            result_preview: rejection.to_string(),
            tokens: ContextTrace::estimate_tokens(rejection),
        });
        tool_results.push(ContentPart::ToolResult {
            tool_call_id: call_id.clone(),
            content: rejection.to_string(),
        });
    }
}
