//! Retried OpenAI-compatible thinker completion.
use super::pace::backoff;
use super::status::classify;
use super::{HttpObserver, ThinkerConfig, ThinkerOutput};
use super::{response as openai_backend_response, send as openai_backend_send};
use crate::thinker::protocol::openai_request as openai_backend_request;
use anyhow::{Result, anyhow};
use reqwest::Client;
use std::time::Instant;

/// Retry once on transient failures (connection errors, 429, 502-504).
const MAX_ATTEMPTS: u32 = 2;

/// Generate a thought via an OpenAI-compatible chat-completions endpoint.
///
/// # Errors
///
/// Returns the last transient error after `MAX_ATTEMPTS`, or immediately for
/// non-retryable transport and status failures.
///
/// # Arguments
/// * `config` - Endpoint, model, sampling and optional bearer credential settings.
/// * `http` - Client configured by the host.
/// * `system_prompt` - Caller system instructions.
/// * `user_prompt` - Caller user text.
/// * `observer` - Host diagnostics at each original retry/response point.
///
/// # Returns
/// The decoded thinker output when a request succeeds.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::thinker::{ThinkerConfig, http::{think, HttpObserver}};
/// # async fn example(config: &ThinkerConfig, http: &reqwest::Client, observer: &impl HttpObserver) {
/// let output = think(config, http, "Answer concisely", "Hello", observer).await.unwrap();
/// assert!(!output.model.is_empty());
/// # }
/// ```
pub async fn think(
    config: &ThinkerConfig,
    http: &Client,
    system_prompt: &str,
    user_prompt: &str,
    observer: &impl HttpObserver,
) -> Result<ThinkerOutput> {
    let started_at = Instant::now();
    let body = openai_backend_request::body(config, system_prompt, user_prompt);
    let mut last_err: Option<anyhow::Error> = None;

    for attempt in 0..MAX_ATTEMPTS {
        backoff(attempt, observer).await;
        let attempted = match openai_backend_send::send(config, http, &body, observer).await? {
            Ok(response) => classify(response, observer).await?,
            Err(error) => Err(error),
        };
        match attempted {
            Ok(response) => {
                let output = openai_backend_response::decode(config, response).await?;
                observer.success(&output, started_at, attempt);
                return Ok(output);
            }
            Err(error) => last_err = Some(error),
        }
    }

    Err(last_err
        .unwrap_or_else(|| anyhow!("thinker HTTP request failed after {MAX_ATTEMPTS} attempts")))
}
