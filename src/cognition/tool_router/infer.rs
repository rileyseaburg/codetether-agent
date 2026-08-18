//! FunctionGemma inference execution.

use anyhow::{Result, anyhow};
use std::sync::Arc;

use super::parsed_call::ParsedToolCall;
use super::{ToolCallRouter, parse, prioritize, prompt};
use crate::provider::ToolDefinition;

impl ToolCallRouter {
    /// Run the FunctionGemma model on a blocking thread.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime mutex is poisoned, the blocking task
    /// panics, or inference fails.
    pub(super) async fn run_functiongemma(
        &self,
        assistant_text: &str,
        tools: &[ToolDefinition],
    ) -> Result<Vec<ParsedToolCall>> {
        let sorted = prioritize::prioritize(assistant_text, tools);
        let prompt = prompt::build_functiongemma_prompt(assistant_text, &sorted);
        tracing::debug!(prompt_len = prompt.len(), "FunctionGemma prompt built");

        let runtime = Arc::clone(&self.runtime);
        let output = tokio::task::spawn_blocking(move || {
            let mut guard = runtime
                .lock()
                .map_err(|_| anyhow!("FunctionGemma mutex poisoned"))?;
            // `think_raw` skips chat templating: the prompt is already formatted.
            guard.think_raw(&prompt)
        })
        .await
        .map_err(|e| anyhow!("FunctionGemma task join failed: {e}"))??;

        tracing::debug!(
            raw_output = %output.text,
            completion_tokens = output.completion_tokens.unwrap_or(0),
            "FunctionGemma raw output"
        );
        Ok(parse::parse_functiongemma_response(&output.text))
    }
}
