//! Application logging adapter for shared thinker HTTP transport.
use super::{ThinkerConfig, ThinkerOutput};
use anyhow::Result;
use codetether_cognition_core::thinker::http::{self, HttpObserver};
use reqwest::Client;
use std::time::Instant;

/// Generate a thought via an OpenAI-compatible chat-completions endpoint.
///
/// # Errors
///
/// Returns the last transient error after `MAX_ATTEMPTS`, or immediately for
/// non-retryable transport and status failures.
pub(super) async fn think(
    config: &ThinkerConfig,
    http: &Client,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    http::think(config, http, system_prompt, user_prompt, &HttpLog).await
}

struct HttpLog;
impl HttpObserver for HttpLog {
    fn transient_transport(&self, error: &reqwest::Error) {
        super::openai_backend_send::log_transient(error);
    }
    fn transient_status(&self, status: reqwest::StatusCode) {
        super::openai_backend_status::log_transient(status);
    }
    fn retry(&self, attempt: u32) {
        super::openai_backend_trace::log_retry(attempt);
    }
    fn success(&self, output: &ThinkerOutput, started_at: Instant, attempt: u32) {
        super::openai_backend_trace::log_success(output, started_at, attempt);
    }
}
