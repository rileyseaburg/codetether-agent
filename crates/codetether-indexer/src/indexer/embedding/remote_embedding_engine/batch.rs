//! Focused embedding operation.

use super::*;

impl RemoteEmbeddingEngine {
    pub(in crate::indexer) async fn embed_batch(
        &self,
        inputs: &[String],
    ) -> Result<codetether_provider_core::EmbeddingResponse> {
        if inputs.is_empty() {
            return Ok(codetether_provider_core::EmbeddingResponse {
                embeddings: Vec::new(),
                usage: codetether_provider_core::Usage::default(),
            });
        }

        let mut attempt = 0u32;
        loop {
            let request = codetether_provider_core::EmbeddingRequest {
                model: self.model.clone(),
                inputs: inputs.to_vec(),
            };

            match self.provider.embed(request).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    let should_retry =
                        attempt < self.max_retries && is_retryable_embedding_error(&err);
                    if !should_retry {
                        return Err(anyhow::anyhow!(
                            "remote embedding failed via {}/{} after {} attempt(s): {}",
                            self.provider_name,
                            self.model,
                            attempt + 1,
                            err
                        ));
                    }

                    let delay = retry_delay(attempt, self.retry_initial, self.retry_max);
                    tracing::warn!(
                        provider = %self.provider_name,
                        model = %self.model,
                        attempt = attempt + 1,
                        retry_in_ms = delay.as_millis(),
                        error = %err,
                        "Embedding batch failed, retrying"
                    );

                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }
}
