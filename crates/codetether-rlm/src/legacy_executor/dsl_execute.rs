//! Record the assistant text and execute its legacy DSL commands.
use super::*;

impl RlmExecutor {
    pub(super) async fn execute_dsl_step(
        &mut self,
        response: &codetether_provider_core::CompletionResponse,
        messages: &mut Vec<Message>,
        iterations: usize,
    ) -> Result<(String, ReplResult)> {
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

        // Add assistant message
        messages.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: assistant_text.clone(),
            }],
        });

        // Extract and execute code blocks
        let code = self.extract_code(&assistant_text);
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: format!("execute_code({})", truncate_with_ellipsis(&code, 160)),
            output: String::new(),
        });

        // Display execution details in verbose mode
        if self.verbose {
            println!("[RLM] Iteration {}: Executing code:\n{}", iterations, code);
        }

        let execution_result = self.execute_with_llm_query(&code).await?;

        self.log_dsl_result(&execution_result);
        Ok((code, execution_result))
    }
}
