//! HTTP client creation for the A2A worker.

use anyhow::{Context, Result};
use reqwest::Client;

use codetether_provider_core::socket_opts::apply_socket_opts;

pub fn build_worker_http_client() -> Result<Client> {
    apply_socket_opts(
        Client::builder()
            .no_gzip()
            .no_brotli()
            .no_zstd()
            .no_deflate(),
    )
    .build()
    .context("Failed to build worker HTTP client")
}
