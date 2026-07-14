//! Complete provider calls with derived-context retry.

use std::sync::Arc;

use anyhow::Result;

use super::options::RequestOptions;
use super::request::build_request_with_context;

#[path = "complete_recovery.rs"]
mod complete_recovery;
use crate::provider::{CompletionResponse, Provider, ToolDefinition};
use crate::session::Session;

/// Complete a derived-context request, prioritizing prepared RLM recovery.
pub async fn complete_with_context(
    provider: Arc<dyn Provider>,
    session: &Session,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    mut opts: RequestOptions,
) -> Result<CompletionResponse> {
    let mut attempt = 1usize;
    loop {
        let request = build_request_with_context(
            Arc::clone(&provider),
            session,
            model,
            system_prompt,
            tools,
            opts,
        )
        .await?;
        match provider.complete(request).await {
            Ok(response) => return Ok(response),
            Err(error) => {
                let policy = opts
                    .policy_override
                    .unwrap_or_else(|| super::policy::effective_policy(session, model));
                let Some(next) = complete_recovery::next(&error, attempt, policy, opts) else {
                    return Err(error);
                };
                opts = next;
                attempt += 1;
            }
        }
    }
}
