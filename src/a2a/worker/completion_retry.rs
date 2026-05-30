//! Completion fallback logic when tool schemas overflow context.

use std::sync::Arc;

use anyhow::{Context, Result};

use crate::{provider::Provider, session::Session};

use super::{compact_worker_tool_definitions, tool_schema_bytes};

pub(super) async fn complete_worker_step_with_context_fallback(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tool_definitions: &[crate::provider::ToolDefinition],
    temperature: Option<f32>,
) -> Result<crate::provider::CompletionResponse> {
    let options = crate::session::context::RequestOptions {
        temperature,
        top_p: None,
        max_tokens: Some(8192),
        force_keep_last: None,
    };
    match crate::session::context::complete_with_context(
        Arc::clone(&provider),
        session,
        model,
        system_prompt,
        tool_definitions,
        options,
    )
    .await
    {
        Ok(response) => Ok(response),
        Err(error) if crate::session::helper::error::is_prompt_too_long_error(&error) => {
            let compact = compact_worker_tool_definitions(tool_definitions);
            tracing::warn!(error = %error, tool_count = tool_definitions.len(), original_tool_schema_bytes = tool_schema_bytes(tool_definitions), compact_tool_schema_bytes = tool_schema_bytes(&compact), "Provider rejected prompt after context compaction; retrying with compact tool schemas");
            crate::session::context::complete_with_context(provider, session, model, system_prompt, &compact, options).await.map_err(|retry| {
                if crate::session::helper::error::is_prompt_too_long_error(&retry) {
                    retry.context("provider rejected prompt as too long after RLM compaction and compact tool-schema fallback")
                } else {
                    retry
                }
            })
        }
        Err(error) => Err(error),
    }
}
