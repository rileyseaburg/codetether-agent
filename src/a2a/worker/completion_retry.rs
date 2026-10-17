//! Session-context adapter for shared completion fallback policy.
use crate::{
    provider::{CompletionResponse, Provider, ToolDefinition},
    session::{Session, context::RequestOptions},
};
use anyhow::Result;
use std::sync::Arc;
#[path = "completion_retry_host.rs"]
mod host;
struct ApplicationHost<'a> {
    provider: Arc<dyn Provider>,
    session: &'a Session,
    model: &'a str,
    system_prompt: &'a str,
    options: RequestOptions,
}
pub(super) async fn complete_worker_step_with_context_fallback(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tool_definitions: &[ToolDefinition],
    temperature: Option<f32>,
) -> Result<CompletionResponse> {
    let options = RequestOptions {
        temperature,
        top_p: None,
        max_tokens: Some(8192),
        force_keep_last: None,
        policy_override: None,
    };
    codetether_a2a_worker_core::session_protocol::completion::complete_with_fallback(
        ApplicationHost {
            provider,
            session,
            model,
            system_prompt,
            options,
        },
        tool_definitions,
    )
    .await
}
