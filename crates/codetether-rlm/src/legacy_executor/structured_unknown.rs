//! Record a tool name that the RLM dispatcher does not recognize.
use super::*;
impl RlmExecutor {
    pub(super) fn record_structured_unknown(
        &mut self,
        call: &(String, String, String),
        iterations: usize,
        tool_results: &mut Vec<ContentPart>,
    ) {
        let (call_id, name, arguments) = call;

        let unknown = format!("Unknown tool: {name}");
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: format!("{name}({})", truncate_with_ellipsis(arguments, 120)),
            output: unknown.clone(),
        });
        tool_results.push(ContentPart::ToolResult {
            tool_call_id: call_id.clone(),
            content: unknown,
        });
    }
}
