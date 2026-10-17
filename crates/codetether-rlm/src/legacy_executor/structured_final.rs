//! Accept a final tool response only when evidence requirements are satisfied.
use super::*;
impl RlmExecutor {
    pub(super) fn accept_structured_final(
        &mut self,
        call: &(String, String, String),
        answer: String,
        final_answer: &mut Option<String>,
        tool_results: &mut Vec<ContentPart>,
        iterations: usize,
        requires_pattern_evidence: bool,
    ) -> bool {
        let (call_id, name, arguments) = call;

        if requires_pattern_evidence && !self.has_pattern_evidence() {
            self.reject_structured_final(call, iterations, tool_results);
            return false;
        }
        if self.verbose {
            println!("[RLM] Final answer received via tool call");
        }
        *final_answer = Some(answer.clone());
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: format!("{name}({})", truncate_with_ellipsis(arguments, 120)),
            output: format!("FINAL: {}", truncate_with_ellipsis(&answer, 240)),
        });
        self.context_trace.log_event(ContextEvent::Final {
            answer: truncate_with_ellipsis(&answer, 400),
            tokens: ContextTrace::estimate_tokens(&answer),
        });
        tool_results.push(ContentPart::ToolResult {
            tool_call_id: call_id.clone(),
            content: format!("FINAL: {answer}"),
        });
        true
    }
}
