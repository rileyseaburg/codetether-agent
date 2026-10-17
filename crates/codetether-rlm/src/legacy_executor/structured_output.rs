//! Record ordinary in-memory tool output and append its tool message.
use super::*;
impl RlmExecutor {
    pub(super) fn record_structured_output(
        &mut self,
        call: &(String, String, String),
        output: String,
        iterations: usize,
        tool_results: &mut Vec<ContentPart>,
    ) {
        let (call_id, name, arguments) = call;
        // Normal REPL output
        if self.verbose {
            let preview = truncate_with_ellipsis(&output, 200);
            println!("[RLM] Tool {name} → {}", preview);
        }
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: format!("{name}({})", truncate_with_ellipsis(arguments, 120)),
            output: Self::trace_output_for_storage(&output),
        });
        self.context_trace.log_event(ContextEvent::ToolResult {
            tool_call_id: call_id.clone(),
            result_preview: truncate_with_ellipsis(&output, 300),
            tokens: ContextTrace::estimate_tokens(&output),
        });
        tool_results.push(ContentPart::ToolResult {
            tool_call_id: call_id.clone(),
            content: output,
        });
    }
}
