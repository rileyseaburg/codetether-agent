//! Forward the prepared request to the native context service.
use super::*;
use codetether_agent_completion::Request;
#[async_trait::async_trait]
impl<H: Host> codetether_agent_completion::Completion for Agent<H> {
    async fn complete(
        request: Request<'_, H::Session>,
    ) -> anyhow::Result<codetether_provider_core::CompletionResponse> {
        H::complete(request).await
    }
}
