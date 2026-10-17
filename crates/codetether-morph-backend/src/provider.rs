//! Resolve the configured model and submit the editing completion.
use anyhow::{Context, Result};
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
pub(super) async fn apply_with_provider<F, R>(prompt: &str, resolve: F) -> Result<String>
where
    F: FnOnce(String) -> R + Send,
    R: Future<Output = Result<(Arc<dyn Provider>, String)>> + Send,
{
    let model_ref = super::config::morph_model_ref();
    let (provider, model) = resolve(model_ref).await?;
    tracing::info!(target: "codetether_agent::tool::morph_backend",
        provider = %provider.name(),
        model = %model,
        "Executing Morph-backed edit via provider registry"
    );

    let request = super::provider_request::build(prompt, model);
    let response = provider
        .complete(request)
        .await
        .context("Morph provider request failed")?;

    super::provider_response::content(response)
}
