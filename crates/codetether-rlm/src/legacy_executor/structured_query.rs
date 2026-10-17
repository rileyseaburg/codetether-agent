//! Execute and record the asynchronous sub-query sentinel.
use super::*;
impl RlmExecutor {
    pub(super) async fn execute_structured_query(
        &mut self,
        call: &(String, String, String),
        output: &str,
        iterations: usize,
        tool_results: &mut Vec<ContentPart>,
    ) -> Result<bool> {
        let call_id = &call.0;
        if let Ok(sentinel) = serde_json::from_str::<serde_json::Value>(output)
            && sentinel
                .get("__rlm_llm_query")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        {
            let q = sentinel.get("query").and_then(|v| v.as_str()).unwrap_or("");
            let ctx_slice = sentinel
                .get("context_slice")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let llm_result = self.handle_llm_query_direct(q, ctx_slice).await?;
            self.trace_steps.push(TraceStep {
                iteration: iterations,
                action: format!("llm_query({})", truncate_with_ellipsis(q, 120)),
                output: truncate_with_ellipsis(&llm_result, 240),
            });
            self.context_trace.log_event(ContextEvent::ToolResult {
                tool_call_id: call_id.clone(),
                result_preview: truncate_with_ellipsis(&llm_result, 300),
                tokens: ContextTrace::estimate_tokens(&llm_result),
            });
            tool_results.push(ContentPart::ToolResult {
                tool_call_id: call_id.clone(),
                content: llm_result,
            });
            return Ok(true);
        }
        Ok(false)
    }
}
