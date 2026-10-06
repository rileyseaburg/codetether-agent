//! Exercise the production trait default, not a copied model resolver.
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use anyhow::Result;
use futures::stream::BoxStream;

struct AdapterIdentity<'a>(&'a str);

#[async_trait::async_trait]
impl Provider for AdapterIdentity<'_> {
    fn name(&self) -> &str {
        self.0
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        anyhow::bail!("identity-only test must not list models")
    }
    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("identity-only test must not dispatch requests")
    }
    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("identity-only test must not dispatch requests")
    }
}

fn resolve(provider: &str, model: &str) -> String {
    AdapterIdentity(provider).resolved_model_identity(model)
}

#[path = "../../src/provider/routing_identity/tests.rs"]
mod tests;

#[test]
fn current_harness_identity_is_not_rewritten() {
    assert_eq!(resolve("openai-codex", "gpt-6.1-sol"), "gpt-6.1-sol");
}
