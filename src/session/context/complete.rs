//! Complete provider calls with derived-context retry.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::{CompletionResponse, Provider, ToolDefinition};
use crate::session::Session;
use crate::session::helper::prompt_too_long;

use super::options::RequestOptions;
use super::request::build_request_with_context;

/// Complete a derived-context request, retrying with forced compaction.
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
                let Some(keep_last) = prompt_too_long::keep_last(&error, attempt) else {
                    return Err(error);
                };
                opts = RequestOptions {
                    force_keep_last: Some(keep_last),
                    ..opts
                };
                attempt += 1;
            }
        }
    }
}
