//! Bind goal/peer prompts, session input enrichment and native context completion.
use super::*;
use codetether_agent_completion::Request;
#[async_trait::async_trait]
impl codetether_agent_runtime::Context for Native {
    fn compose(base: &str, session: &Session) -> String {
        let prompt = crate::session::tasks::runtime::compose(base, &session.id);
        crate::a2a::peer_prompt::append(prompt)
    }
    fn enrich(input: &Value, cwd: &Path, session: &Session) -> Value {
        crate::session::helper::runtime::enrich_tool_input_for_session(input, cwd, session)
    }
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
