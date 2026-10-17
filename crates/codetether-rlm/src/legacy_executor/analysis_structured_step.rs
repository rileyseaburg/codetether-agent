//! Handle a structured-tool response and report whether it was consumed.
use super::*;
impl RlmExecutor {
    pub(super) async fn execute_structured_step(
        &mut self,
        response: &codetether_provider_core::CompletionResponse,
        run: &mut AnalysisRun,
    ) -> Result<bool> {
        let tool_calls: Vec<(String, String, String)> = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } => Some((id.clone(), name.clone(), arguments.clone())),
                _ => None,
            })
            .collect();

        if !tool_calls.is_empty() {
            self.dispatch_structured_batch(
                response,
                &tool_calls,
                &mut run.messages,
                run.iterations,
                run.requires_pattern_evidence,
                &mut run.final_answer,
            )
            .await?;
            return Ok(true);
        }

        Ok(false)
    }
}
