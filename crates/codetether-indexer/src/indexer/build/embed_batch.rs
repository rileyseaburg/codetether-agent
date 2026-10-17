//! embed batch for index construction.

use super::*;

pub(super) async fn embed_batch(
    backend: &EmbeddingBackend,
    embedding_slice: &[String],
    start: usize,
    end: usize,
) -> Result<(Vec<Vec<f32>>, (u64, u64))> {
    Ok(match backend {
        EmbeddingBackend::Local { engine, .. } => {
            let vectors = engine.embed_batch(embedding_slice);
            let mut local_prompt_tokens = 0u64;
            let mut local_total_tokens = 0u64;
            for input in embedding_slice {
                let approx_tokens = approximate_token_count(input);
                local_prompt_tokens += approx_tokens;
                local_total_tokens += approx_tokens;
            }
            (vectors, (local_prompt_tokens, local_total_tokens))
        }
        EmbeddingBackend::Remote(engine) => {
            let response = engine.embed_batch(embedding_slice).await.with_context(|| {
                format!(
                    "failed embedding batch {}-{} via provider {}/{}",
                    start, end, engine.provider_name, engine.model
                )
            })?;

            let vectors = response.embeddings;
            let prompt_tokens = response.usage.prompt_tokens as u64;
            let total_tokens = response.usage.total_tokens as u64;
            (vectors, (prompt_tokens, total_tokens))
        }
    })
}
