//! Record ordinary DSL output and append the next user turn.
use super::*;

impl RlmExecutor {
    pub(super) fn record_dsl_output(
        &mut self,
        execution_result: &ReplResult,
        iterations: usize,
        messages: &mut Vec<Message>,
    ) {
        self.context_trace.log_event(ContextEvent::ExecutionOutput {
            output: truncate_with_ellipsis(&execution_result.stdout, 400),
            tokens: ContextTrace::estimate_tokens(&execution_result.stdout),
        });
        if let Some(step) = self.trace_steps.last_mut()
            && step.iteration == iterations
            && step.output.is_empty()
        {
            step.output = Self::trace_output_for_storage(&execution_result.stdout);
        }

        // Add execution result as user message for next iteration
        let result_text = if execution_result.stdout.is_empty() {
            "[No output]".to_string()
        } else {
            format!("Execution result:\n{}", execution_result.stdout)
        };

        messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: result_text }],
        });
    }
}
