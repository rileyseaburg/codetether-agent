//! Worker registration request handling.

use anyhow::Result;
use reqwest::Client;

use codetether_native_providers::worker_models as provider_models;

pub async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    public_url: Option<&str>,
) -> Result<()> {
    codetether_a2a_http::worker::register_worker(
        client,
        server,
        token,
        worker_id,
        name,
        codebases,
        public_url,
        provider_models::load_provider_models(),
    )
    .await
}

/// Register the current worker using its captured startup context.
pub async fn register_current_worker(context: &crate::WorkerContext) -> Result<()> {
    codetether_a2a_http::worker::register_context(
        context,
        &context.args.token,
        context.args.public_url.as_deref(),
        codetether_native_providers::worker_models::load_provider_models(),
    )
    .await
}
