//! Bedrock completion service with host-owned signed dispatch.
//!
//! [think] constructs the payload and decodes the response. The host supplies
//! signed request dispatch and success observation; no credentials are loaded here.
use super::protocol::bedrock_request;
use super::{ThinkerConfig, ThinkerOutput};
use anyhow::{Context, Result};
use std::future::Future;
use std::time::Instant;
mod decode;

/// Generate a thought through a caller-supplied signed Converse request.
///
/// # Arguments
/// * `config` - Model, region and generation settings.
/// * `system_prompt` - Caller system instructions.
/// * `user_prompt` - Caller user text.
/// * `send` - Signed sender receiving the URL and serialized request body.
/// * `on_success` - Observer called after decoding with the operation start time.
///
/// # Returns
/// Normalized thinker output.
///
/// # Errors
/// Returns serialization, dispatch, status, body-read or response-decoding errors.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::thinker::{ThinkerConfig, bedrock::think};
/// # use std::future::Future;
/// # async fn example<F>(config: &ThinkerConfig, send: impl FnOnce(String, Vec<u8>) -> F)
/// # where F: Future<Output = anyhow::Result<reqwest::Response>> {
/// let _output = think(config, "Answer concisely", "Hello", send, |_, _| {}).await.unwrap();
/// # }
/// ```
pub async fn think<F>(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
    send: impl FnOnce(String, Vec<u8>) -> F,
    on_success: impl FnOnce(&ThinkerOutput, Instant),
) -> Result<ThinkerOutput>
where
    F: Future<Output = Result<reqwest::Response>>,
{
    let started_at = Instant::now();
    let body = bedrock_request::body(config, system_prompt, user_prompt);
    let body_bytes = serde_json::to_vec(&body)?;
    let response = send(bedrock_request::url(config), body_bytes)
        .await
        .context("Bedrock thinker converse request failed")?;
    let output = decode::decode(config, response).await?;
    on_success(&output, started_at);
    Ok(output)
}
