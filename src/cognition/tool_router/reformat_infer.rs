//! FunctionGemma inference path with timeout and error recovery.

use std::time::Duration;

use super::{ToolCallRouter, rewrite};
use crate::provider::{CompletionResponse, ToolDefinition};

/// Inference budget. GPU runs take roughly 4s, CPU runs roughly 65s.
const TIMEOUT: Duration = Duration::from_secs(90);

impl ToolCallRouter {
    /// Run inference and rewrite on success; return `response` unchanged on any
    /// timeout, error, or empty result.
    pub(super) async fn reformat_via_inference(
        &self,
        response: CompletionResponse,
        assistant_text: &str,
        tools: &[ToolDefinition],
    ) -> CompletionResponse {
        tracing::info!(
            num_tools = tools.len(),
            "Running FunctionGemma tool extraction"
        );
        match tokio::time::timeout(TIMEOUT, self.run_functiongemma(assistant_text, tools)).await {
            Ok(Ok(parsed)) if !parsed.is_empty() => {
                tracing::info!(
                    num_calls = parsed.len(),
                    "FunctionGemma router produced tool calls from text-only response"
                );
                rewrite::rewrite_response(response, parsed)
            }
            // FunctionGemma decided no tool calls are needed.
            Ok(Ok(_)) => response,
            Ok(Err(error)) => {
                tracing::warn!(%error, "FunctionGemma router failed; returning original response");
                response
            }
            Err(_elapsed) => {
                tracing::warn!("FunctionGemma timed out after 90s; returning original response");
                response
            }
        }
    }
}
