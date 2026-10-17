//! Preserve assistant history and collect an ordered batch of tool results.
use super::*;
impl RlmExecutor {
    pub(super) async fn dispatch_structured_batch(
        &mut self,
        response: &codetether_provider_core::CompletionResponse,
        tool_calls: &[(String, String, String)],
        messages: &mut Vec<Message>,
        iterations: usize,
        requires_pattern_evidence: bool,
        final_answer: &mut Option<String>,
    ) -> Result<()> {
        tracing::info!(
            count = tool_calls.len(),
            "RLM: dispatching structured tool calls"
        );

        // Keep the assistant message (including tool call parts) in history
        messages.push(Message {
            role: Role::Assistant,
            content: response.message.content.clone(),
        });

        let mut tool_results: Vec<ContentPart> = Vec::new();

        for call in tool_calls {
            if self
                .dispatch_structured_call(
                    call,
                    iterations,
                    requires_pattern_evidence,
                    final_answer,
                    &mut tool_results,
                )
                .await?
            {
                break;
            }
        }

        // Send tool results back as a Role::Tool message
        if !tool_results.is_empty() {
            messages.push(Message {
                role: Role::Tool,
                content: tool_results,
            });
        }

        Ok(())
    }
}
