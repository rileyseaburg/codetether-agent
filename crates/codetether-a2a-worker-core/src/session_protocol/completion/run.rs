//! One compact-schema retry for context-overflow failures.
use super::CompletionHost;
use anyhow::Result;
use codetether_context_policy::request::recovery::is_prompt_too_long_error;
use codetether_provider_core::{
    CompletionResponse, ToolDefinition,
    tool_schema::{compact_worker_tool_definitions, tool_schema_bytes},
};
/// Attempt completion and retry context overflow once with compacted schemas.
///
/// # Arguments
/// * `host` - Provider/session completion context.
/// * `tool_definitions` - Original tool schemas for the first attempt.
/// # Returns
/// The first successful response, including a successful compact-schema retry.
/// # Errors
/// Propagates ordinary errors and annotates a repeated context-overflow error.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: impl codetether_a2a_worker_core::session_protocol::completion::CompletionHost) -> anyhow::Result<()> {
/// let response = codetether_a2a_worker_core::session_protocol::completion::complete_with_fallback(host, &[]).await?;
/// # Ok(()) }
/// ```
pub async fn complete_with_fallback<H: CompletionHost>(
    mut host: H,
    tool_definitions: &[ToolDefinition],
) -> Result<CompletionResponse> {
    match host.complete_initial(tool_definitions).await {
        Ok(response) => Ok(response),
        Err(error) if is_prompt_too_long_error(&error) => {
            let compact = compact_worker_tool_definitions(tool_definitions);
            tracing::warn!(target: "codetether_agent::a2a::worker::completion_retry", error = %error, tool_count = tool_definitions.len(), original_tool_schema_bytes = tool_schema_bytes(tool_definitions), compact_tool_schema_bytes = tool_schema_bytes(&compact), "Provider rejected prompt after prepared-context recovery; retrying with compact tool schemas");
            host.complete_retry(&compact).await.map_err(|retry| {
                if is_prompt_too_long_error(&retry) {
                    retry.context("provider rejected prompt as too long after RLM compaction and compact tool-schema fallback")
                } else {
                    retry
                }
            })
        }
        Err(error) => Err(error),
    }
}
