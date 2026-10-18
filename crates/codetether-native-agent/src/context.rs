//! Native goal/peer prompt composition and context-aware completion.
use super::{Host, SessionServices};
use codetether_agent_completion::Request;
use codetether_application_registry::Application;
use codetether_context_request::Source;
use codetether_native_context::{ContextHost, RequestOptions};
use serde_json::Value;
use std::path::Path;
#[async_trait::async_trait]
impl<S: SessionServices, T: Application> codetether_agent_runtime::Context for Host<S, T>
where
    S::Session: ContextHost,
{
    fn compose(base: &str, session: &Self::Session) -> String {
        let prompt = codetether_native_goals::runtime::compose(base, session.session_id());
        codetether_a2a_discovery::peer_prompt::append(prompt)
    }
    fn enrich(input: &Value, cwd: &Path, session: &Self::Session) -> Value {
        S::enrich(input, cwd, session)
    }
    async fn complete(
        request: Request<'_, Self::Session>,
    ) -> anyhow::Result<codetether_provider_core::CompletionResponse> {
        codetether_native_context::complete_with_context(
            request.provider,
            request.session,
            request.model,
            request.system_prompt,
            request.tools,
            RequestOptions {
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
