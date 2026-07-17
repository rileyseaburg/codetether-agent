//! Stream-completion collection with incremental event forwarding.
//!
//! [`collect_stream_completion_with_events`] drains a provider stream,
//! accumulates text and tool-call deltas into a final
//! [`CompletionResponse`](crate::provider::CompletionResponse), and (optionally)
//! forwards incremental [`SessionEvent::TextChunk`] snapshots to a UI layer.
//!
//! ## Snapshot truncation
//!
//! Each text chunk forwarded over `event_tx` is a **full snapshot** of the
//! accumulated assistant text so far. For extremely long replies this would be
//! O(n²) in memory; to bound the worst case the snapshot is capped at
//! [`stream_caps::MAX_STREAM_SNAPSHOT_BYTES`](super::stream_caps::MAX_STREAM_SNAPSHOT_BYTES) with a trailing `" …[truncated]"` marker.
//! The full text is still returned in the final [`CompletionResponse`]; only
//! the streamed previews are truncated.

use super::super::SessionEvent;
use crate::provider::StreamChunk;
use anyhow::Result;
use futures::stream::BoxStream;

mod accept;
#[cfg(test)]
mod drain_end_tests;
mod drain_state;
mod fault;
#[cfg(test)]
mod fault_tests;
mod finalize;
mod idle_drain;
mod idle_fault;
mod idle_keepalive;
mod idle_timeout;
mod outcome;
mod reasoning;
#[cfg(test)]
mod reasoning_signature_tests;
mod restart;
#[cfg(test)]
mod restart_policy_tests;
#[cfg(test)]
mod restart_test_provider;
#[cfg(test)]
mod restart_tests;
#[cfg(test)]
mod srp_tests;
#[cfg(test)]
mod stall_restart_tests;
#[cfg(test)]
mod stall_test_provider;
mod text_acc;
#[cfg(test)]
mod thinking_tests;
mod tool_acc;

pub(crate) use restart::{RestartPolicy, run as run_with_restart};

/// Collect a streaming completion into a [`CompletionResponse`](crate::provider::CompletionResponse),
/// optionally forwarding incremental events.
///
/// Reads [`StreamChunk`]s from `stream`, accumulates assistant text,
/// thinking/reasoning deltas, and tool-call argument deltas keyed by
/// tool-call id, and tracks the final
/// [`FinishReason`](crate::provider::FinishReason) and [`Usage`]. Thinking
/// deltas are preserved as a leading
/// [`ContentPart::Thinking`](crate::provider::ContentPart) so a
/// thinking-only completion never yields an empty assistant message.
/// When `event_tx` is `Some`, each text delta
/// triggers a [`SessionEvent::TextChunk`] carrying the full accumulated text
/// up to that point — truncated to [`stream_caps::MAX_STREAM_SNAPSHOT_BYTES`](super::stream_caps::MAX_STREAM_SNAPSHOT_BYTES) with a
/// `" …[truncated]"` suffix when exceeded.
///
/// # Arguments
///
/// * `stream` — Boxed async stream of [`StreamChunk`]s from a provider.
/// * `event_tx` — Optional channel for UI preview events; pass `None` for
///   headless/non-interactive callers.
///
/// # Returns
///
/// A fully materialized [`CompletionResponse`](crate::provider::CompletionResponse)
/// containing the complete assistant text and any accumulated tool calls.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the stream yields a terminal error chunk,
/// response assembly fails, or the stream ends without assistant content.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::session::helper::stream::collect_stream_completion_with_events;
/// use futures::stream;
///
/// // In practice the stream comes from a Provider::stream() call.
/// let s = Box::pin(stream::iter([codetether_agent::provider::StreamChunk::Text(
///     "ok".into(),
/// )]));
/// let response = collect_stream_completion_with_events(s, None).await.unwrap();
/// // `response` is a CompletionResponse; inspect it as needed.
/// let _ = response;
/// # });
/// ```
pub async fn collect_stream_completion_with_events(
    stream: BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> Result<crate::provider::CompletionResponse> {
    let outcome = idle_timeout::drain(stream, event_tx).await;
    accept::accept(outcome, 0)
}
