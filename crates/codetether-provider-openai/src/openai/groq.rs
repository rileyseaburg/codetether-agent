//! Groq chat request validation and optional settings shared by both transports.
use anyhow::{Result, ensure};
use codetether_provider_core::CompletionRequest;
use serde_json::{Value, json};

pub(super) fn validate(request: &CompletionRequest) -> Result<()> {
    ensure!(
        request.tools.len() <= 128,
        "Groq accepts at most 128 function tools"
    );
    ensure!(
        request.stop.len() <= 4,
        "Groq accepts at most four stop sequences"
    );
    if let Some(max) = request.max_tokens {
        ensure!(
            max > 0 && u32::try_from(max).is_ok(),
            "Groq token limit must fit a positive u32"
        );
    }
    if let Some(value) = request.temperature {
        ensure!(
            value.is_finite() && (0.0..=2.0).contains(&value),
            "Groq temperature must be between 0 and 2"
        );
    }
    if let Some(value) = request.top_p {
        ensure!(
            value.is_finite() && (0.0..=1.0).contains(&value),
            "Groq top_p must be between 0 and 1"
        );
    }
    Ok(())
}

pub(super) fn streaming_options(body: &mut Value, request: &CompletionRequest) {
    if !request.stop.is_empty() {
        body["stop"] = json!(request.stop);
    }
}
