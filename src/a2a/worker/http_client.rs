//! HTTP client creation for the A2A worker.

use anyhow::{Context, Result};
use reqwest::Client;

pub(super) fn build_worker_http_client() -> Result<Client> {
    Client::builder()
        .no_gzip()
        .no_brotli()
        .no_zstd()
        .no_deflate()
        .build()
        .context("Failed to build worker HTTP client")
}
