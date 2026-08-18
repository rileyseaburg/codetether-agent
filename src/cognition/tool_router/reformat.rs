//! Conditional response reformatting entry point.

use super::{ToolCallRouter, direct, inspect, rewrite};
use crate::provider::{CompletionResponse, ToolDefinition};

impl ToolCallRouter {
    /// Conditionally reformat a `CompletionResponse`.
    ///
    /// The response is returned unchanged when the model supports native tool
    /// calling, already has structured tool calls, has no tools to match, or has
    /// no assistant text. Otherwise a direct `<tool_call>` parse is tried first,
    /// then FunctionGemma inference. Any internal failure returns the original.
    pub async fn maybe_reformat(
        &self,
        response: CompletionResponse,
        tools: &[ToolDefinition],
        model_supports_tools: bool,
    ) -> CompletionResponse {
        if model_supports_tools {
            tracing::trace!("Skipping tool router: model supports native tool calling");
            return response;
        }
        if inspect::has_tool_calls(&response) || tools.is_empty() {
            return response;
        }
        let assistant_text = inspect::assistant_text(&response);
        if assistant_text.trim().is_empty() {
            return response;
        }

        let direct = direct::direct_calls(&assistant_text, tools);
        if !direct.is_empty() {
            tracing::info!(
                num_calls = direct.len(),
                "Direct parse extracted tool calls from text response"
            );
            return rewrite::rewrite_response(response, direct);
        }
        self.reformat_via_inference(response, &assistant_text, tools)
            .await
    }
}
