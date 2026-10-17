//! populate embeddings for index construction.

use super::*;

pub(super) async fn populate_embeddings(
    data: &mut IndexInputs,
    options: &BuildOptions,
    resolver: &dyn EmbeddingResolver,
) -> Result<(String, String)> {
    if !options.include_embeddings {
        return Ok((
            DISABLED_EMBEDDING_PROVIDER.to_string(),
            DISABLED_EMBEDDING_MODEL.to_string(),
        ));
    }
    let backend = resolve_embedding_backend(options, resolver).await?;
    let batch_size = options.embedding_batch_size.max(1);
    data.stats.embedding_dimensions = options.embedding_dimensions.max(64) as u32;

    for start in (0..data.embedding_inputs.len()).step_by(batch_size) {
        let end = (start + batch_size).min(data.embedding_inputs.len());
        let (vectors, usage) =
            embed_batch(&backend, &data.embedding_inputs[start..end], start, end).await?;
        data.stats.embedding_prompt_tokens += usage.0;
        data.stats.embedding_total_tokens += usage.1;
        apply_vectors(data, &backend, start, vectors)?;
    }
    Ok((
        backend.provider_name().to_string(),
        backend.model_name().to_string(),
    ))
}
