//! resolve embedding backend for codebase indexing.

use super::*;

pub(in crate::indexer) async fn resolve_embedding_backend(
    options: &BuildOptions,
    resolver: &dyn EmbeddingResolver,
) -> Result<EmbeddingBackend> {
    let dimensions = options.embedding_dimensions.max(64);
    if is_local_embedding_provider(&options.embedding_provider) {
        return Ok(EmbeddingBackend::Local {
            engine: LocalEmbeddingEngine::new(dimensions),
            model: options.embedding_model.clone(),
        });
    }

    let model_selector =
        build_model_selector(&options.embedding_provider, &options.embedding_model)?;
    let (provider, model) = resolver.resolve(&model_selector).await?;

    let retry_initial = Duration::from_millis(options.embedding_retry_initial_ms.max(1));
    let retry_max = Duration::from_millis(options.embedding_retry_max_ms.max(1));

    Ok(EmbeddingBackend::Remote(RemoteEmbeddingEngine {
        provider_name: provider.name().to_string(),
        provider,
        model,
        max_retries: options.embedding_max_retries,
        retry_initial,
        retry_max,
    }))
}
