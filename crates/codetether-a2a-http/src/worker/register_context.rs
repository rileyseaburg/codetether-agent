//! Registration snapshots from the initialized worker's shared context.
use codetether_a2a_worker_core::initialization::WorkerContext;
use codetether_provider_core::ModelInfo;
use reqwest::Client;
use std::{collections::HashMap, future::Future};

/// Snapshot advertised codebases and submit the current worker registration.
/// # Arguments
/// Receives the initialized context, authentication token, optional public URL
/// and the application's provider-model discovery future.
/// # Returns
/// Unit after the control plane accepts registration.
/// # Errors
/// Returns metadata, proof, transport or registration rejection errors.
/// # Examples
/// ```no_run
/// # async fn example<A, B, P>(
/// # context: &codetether_a2a_worker_core::initialization::WorkerContext<A, reqwest::Client, B, P>) -> anyhow::Result<()> {
/// codetether_a2a_http::worker::register_context(context, &None, None,
///     async { Ok(std::collections::HashMap::new()) }).await
/// # }
/// ```
pub async fn register_context<A, B, P>(
    context: &WorkerContext<A, Client, B, P>,
    token: &Option<String>,
    public_url: Option<&str>,
    models: impl Future<Output = anyhow::Result<HashMap<String, Vec<ModelInfo>>>>,
) -> anyhow::Result<()> {
    let codebases = context.shared_codebases.lock().await.clone();
    super::register_worker(
        &context.task_runtime.client,
        &context.server,
        token,
        &context.worker_id,
        &context.name,
        &codebases,
        public_url,
        models,
    )
    .await
}
