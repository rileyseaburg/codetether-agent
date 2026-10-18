//! Preserve native request options when invoking session-context completion.
use super::*;
use codetether_agent_completion::Request;
#[async_trait::async_trait]
impl codetether_agent_completion::Completion for Agent {
    async fn complete(
        request: Request<'_, Session>,
    ) -> anyhow::Result<crate::provider::CompletionResponse> {
        crate::session::context::complete_with_context(
            request.provider,
            request.session,
            request.model,
            request.system_prompt,
            request.tools,
            crate::session::context::RequestOptions {
                temperature: request.temperature,
                top_p: request.top_p,
                max_tokens: None,
                force_keep_last: None,
                policy_override: None,
            },
        )
        .await
    }
}
