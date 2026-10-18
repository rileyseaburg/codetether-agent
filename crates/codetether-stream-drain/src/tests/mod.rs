//! Original collector regressions using a unit snapshot event type.
use codetether_provider_core::{CompletionResponse, StreamChunk};
use futures::stream::BoxStream;
type SessionEvent = codetether_session_events::SessionEvent<()>;
async fn collect_stream_completion_with_events(
    stream: BoxStream<'static, StreamChunk>,
    tx: Option<&tokio::sync::mpsc::Sender<SessionEvent>>,
) -> anyhow::Result<CompletionResponse> {
    crate::collect_stream_completion_with_events(stream, tx).await
}
mod collector_text;
mod collector_tools;
mod ending_done_finishes_without_waiting_for_transport_close;
mod ending_single_drain_rejects_partial_on_premature_end;
mod ending_stream_ending_with_done_is_clean;
mod fault_accept;
mod signature_opaque_reasoning_signature_reaches_the_next_turn;
mod thinking_empty_thinking_emits_activity_without_content;
mod thinking_thinking_only_stream_produces_thinking_content;
mod thinking_thinking_precedes_text_in_final_content;
