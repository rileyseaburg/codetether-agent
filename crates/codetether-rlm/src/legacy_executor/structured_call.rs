//! Dispatch one structured tool and report whether it ended analysis.
use super::*;
impl RlmExecutor {
    pub(super) async fn dispatch_structured_call(
        &mut self,
        call: &(String, String, String),
        iterations: usize,
        requires_pattern_evidence: bool,
        final_answer: &mut Option<String>,
        tool_results: &mut Vec<ContentPart>,
    ) -> Result<bool> {
        let (_, name, arguments) = call;
        self.context_trace.log_event(ContextEvent::ToolCall {
            name: name.clone(),
            arguments_preview: truncate_with_ellipsis(arguments, 200),
            tokens: ContextTrace::estimate_tokens(arguments),
        });
        match dispatch_tool_call(name, arguments, &mut self.repl) {
            Some(RlmToolResult::Final(answer)) => {
                if self.accept_structured_final(
                    call,
                    answer,
                    final_answer,
                    tool_results,
                    iterations,
                    requires_pattern_evidence,
                ) {
                    return Ok(true);
                }
                return Ok(false);
            }
            Some(RlmToolResult::Output(output)) => {
                if self
                    .execute_structured_query(call, &output, iterations, tool_results)
                    .await?
                {
                    return Ok(false);
                }
                self.record_structured_output(call, output, iterations, tool_results);
            }
            None => {
                self.record_structured_unknown(call, iterations, tool_results);
            }
        }

        Ok(false)
    }
}
