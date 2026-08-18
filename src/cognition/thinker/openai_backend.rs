//! OpenAI-compatible HTTP thinker backend with transient-failure retry.

use anyhow::{Result, anyhow};
use reqwest::Client;
use std::time::Instant;

use super::openai_backend_status::classify;
use super::openai_backend_trace::{backoff, log_success};
use super::{
    ThinkerConfig, ThinkerOutput, openai_backend_request, openai_backend_response,
    openai_backend_send,
};

/// Retry once on transient failures (connection errors, 429, 502-504).
const MAX_ATTEMPTS: u32 = 2;

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
    let started_at = Instant::now();
    let body = openai_backend_request::body(config, system_prompt, user_prompt);
    let mut last_err: Option<anyhow::Error> = None;

    for attempt in 0..MAX_ATTEMPTS {
        backoff(attempt).await;
        let attempted = match openai_backend_send::send(config, http, &body).await? {
            Ok(response) => classify(response).await?,
            Err(error) => Err(error),
        };
        match attempted {
            Ok(response) => {
                let output = openai_backend_response::decode(config, response).await?;
                log_success(&output, started_at, attempt);
                return Ok(output);
            }
            Err(error) => last_err = Some(error),
        }
    }

    Err(last_err
        .unwrap_or_else(|| anyhow!("thinker HTTP request failed after {MAX_ATTEMPTS} attempts")))
}
