//! Hosted vision analyzer: direct streamed completion, no agent loop.
use crate::{Analysis, Analyze, upstream_body::body, upstream_sse::SseParser};
use anyhow::{Context, Result, bail};
use futures_util::StreamExt;
use std::sync::Arc;

/// Create an analyzer posting to `{origin}/v1/chat/completions`.
///
/// Redirects are refused and non-loopback origins must use HTTPS.
///
/// # Errors
/// Returns an error for an insecure origin or HTTP client build failure.
pub fn vision_analyzer(token: &str, origin: &str) -> Result<Analyze> {
    let loopback =
        origin.starts_with("http://127.0.0.1:") || origin.starts_with("http://localhost:");
    if !origin.starts_with("https://") && !loopback {
        bail!("Vision origin must use HTTPS");
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let url = format!("{}/v1/chat/completions", origin.trim_end_matches('/'));
    let auth = format!("Bearer {token}");
    Ok(Arc::new(move |input: Analysis| {
        let request = client.post(&url).header("Authorization", &auth).json(&body(
            &input.model,
            &input.prompt,
            &input.previous,
            &input.image,
        ));
        Box::pin(stream(request, input))
    }))
}
async fn stream(request: reqwest::RequestBuilder, mut input: Analysis) -> Result<()> {
    let response = request.send().await.context("Vision request failed")?;
    let streaming = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("text/event-stream"));
    if !response.status().is_success() || !streaming {
        tracing::warn!(event = "screen_upstream_rejected", requested_model = %input.model, status = response.status().as_u16(), "Vision stream unavailable");
        bail!("Vision stream unavailable");
    }
    let mut parser = SseParser::default();
    let mut chunks = response.bytes_stream();
    while let Some(chunk) = chunks.next().await {
        if input.cancel.is_cancelled() {
            bail!("Analysis cancelled");
        }
        if parser.feed(&chunk?, &mut *input.delta)? {
            return Ok(());
        }
    }
    bail!("Analysis stream ended without DONE")
}
