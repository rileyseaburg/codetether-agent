//! Record ordered batches and simulate transient provider failures.

use super::*;
#[async_trait::async_trait]
impl EmbeddingProvider for Provider {
    fn name(&self) -> &str {
        "fixture-provider"
    }
    async fn embed(&self, request: EmbeddingRequest) -> anyhow::Result<EmbeddingResponse> {
        assert_eq!(request.model, "resolved-model");
        let mut requests = self.calls.requests.lock().unwrap();
        requests.push(request.clone());
        if requests.len() <= self.failures {
            anyhow::bail!("HTTP 429 rate limited");
        }
        Ok(EmbeddingResponse {
            embeddings: vec![vec![0.5; self.dimensions]; request.inputs.len()],
            usage: Usage {
                prompt_tokens: 11,
                total_tokens: 13,
                ..Usage::default()
            },
        })
    }
}
