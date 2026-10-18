//! Upload the JSONL rendering of a complete transcript.
use crate::{HistorySinkConfig, encode_jsonl_delta};
use anyhow::Result;
use codetether_provider_core::Message;

/// Overwrite the session's `history.jsonl` object with the full JSONL
/// rendering of `messages`.
///
/// Simplest semantic: PUT the whole file every time. Delta uploads
/// (append semantics over a chunked layout) are a Phase B follow-up —
/// full-file rewrite keeps the sink single-round-trip and easy to
/// reason about for Phase A.
///
/// # Errors
///
/// Returns the underlying MinIO error verbatim; the caller is expected
/// to log-and-swallow (non-fatal sink) unless it is in a test that
/// actually wants upload failure to surface.
/// # Arguments
/// `config` selects the backend, `session_id` selects its object, and
/// `messages` supplies the full ordered transcript.
/// # Returns
/// Unit after JSONL encoding and a successful upload.
/// # Examples
/// ```rust,no_run
/// # async fn example(config: &codetether_history_sink::HistorySinkConfig) -> anyhow::Result<()> {
/// codetether_history_sink::upload_full_history(config, "session", &[]).await?;
/// # Ok(()) }
/// ```
pub async fn upload_full_history(
    config: &HistorySinkConfig,
    session_id: &str,
    messages: &[Message],
) -> Result<()> {
    crate::upload_encoded::encoded(
        config,
        session_id,
        encode_jsonl_delta(messages, 0)?.into_bytes(),
    )
    .await
}
