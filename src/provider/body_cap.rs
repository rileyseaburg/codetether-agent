//! Bounded HTTP body readers — cap peak memory for provider `/models`
//! (and similar) calls so that a runaway multi-megabyte response cannot
//! OOM the process during startup/registration.

use anyhow::{Context, Result, bail};
use futures::StreamExt;
use reqwest::Response;

/// Default cap for provider catalog / metadata responses. Large enough
/// for realistic `/models` payloads (OpenRouter currently ~0.5 MiB),
/// small enough that an unbounded or hostile response cannot blow the
/// process stack.
pub const PROVIDER_METADATA_BODY_CAP: usize = 8 * 1024 * 1024; // 8 MiB

/// Read an HTTP response body into memory, failing if it exceeds
/// `max_bytes`. Streams chunks so the peak allocation is bounded by
/// `max_bytes + chunk_size`.
pub async fn read_body_capped(resp: Response, max_bytes: usize) -> Result<Vec<u8>> {
    if let Some(len) = resp.content_length()
        && len as usize > max_bytes
    {
        bail!(
            "response body {} B exceeds cap {} B (Content-Length)",
            len,
            max_bytes,
        );
    }
    let mut buf: Vec<u8> = Vec::with_capacity(16 * 1024);
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("read response chunk")?;
        if buf.len().saturating_add(chunk.len()) > max_bytes {
            bail!(
                "response body exceeded cap {} B after {} B",
                max_bytes,
                buf.len(),
            );
        }
        buf.extend_from_slice(&chunk);
    }
    Ok(buf)
}

/// Convenience: fetch JSON with a hard body cap. Returns `T` on success,
/// or an error if the cap is exceeded / parsing fails.
pub async fn json_capped<T: serde::de::DeserializeOwned>(
    resp: Response,
    max_bytes: usize,
) -> Result<T> {
    let bytes = read_body_capped(resp, max_bytes).await?;
    serde_json::from_slice::<T>(&bytes).context("decode capped JSON body")
}
