//! response for the OpenAI-compatible transport.

use super::*;

pub(super) async fn parse_embedding_response(
    response: reqwest::Response,
    expected: usize,
) -> Result<EmbeddingResponse> {
    let status = response.status();
    let text = response.text().await?;
    if !status.is_success() {
        anyhow::bail!(
            "embedding request failed ({status}): {}",
            safe_char_prefix(&text, 500)
        );
    }

    let mut payload: OpenAIEmbeddingResponse = serde_json::from_str(&text)?;
    payload.data.sort_by_key(|item| item.index);
    let embeddings: Vec<Vec<f32>> = payload
        .data
        .into_iter()
        .map(|item| item.embedding)
        .collect();

    if embeddings.len() != expected {
        anyhow::bail!(
            "embedding response length mismatch: expected {}, got {}",
            expected,
            embeddings.len()
        );
    }

    let prompt_tokens = payload.usage.prompt_tokens.unwrap_or(0) as usize;
    let total_tokens = payload
        .usage
        .total_tokens
        .unwrap_or(payload.usage.prompt_tokens.unwrap_or(0)) as usize;

    Ok(EmbeddingResponse {
        embeddings,
        usage: Usage {
            prompt_tokens,
            completion_tokens: 0,
            total_tokens,
            ..Default::default()
        },
    })
}
