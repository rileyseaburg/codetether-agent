//! Tests for how the single-drain collector treats stream termination.
//!
//! Distinct from the restart engine (see `srp_tests`): this leaf collector has
//! no restart budget, so it keeps any committed partial and only errors when
//! nothing was emitted.

use super::collect_stream_completion_with_events;
use crate::provider::{ContentPart, StreamChunk};
use futures::stream;

#[tokio::test]
async fn single_drain_keeps_committed_partial_on_premature_end() {
    // The single-drain collector has no restart budget, so `accept` returns any
    // committed content even when the stream ended before `Done`. Discarding the
    // partial and re-requesting is the *restart engine's* job (see srp_tests),
    // not this leaf collector's.
    let s = Box::pin(stream::iter(vec![StreamChunk::Text("partial".to_string())]));
    let resp = collect_stream_completion_with_events(s, None)
        .await
        .unwrap();
    assert!(matches!(
        &resp.message.content[0],
        ContentPart::Text { text } if text == "partial"
    ));
}

#[tokio::test]
async fn stream_ending_with_done_is_clean() {
    let s = Box::pin(stream::iter(vec![
        StreamChunk::Text("complete".to_string()),
        StreamChunk::Done { usage: None },
    ]));
    let resp = collect_stream_completion_with_events(s, None)
        .await
        .unwrap();
    assert!(matches!(
        &resp.message.content[0],
        ContentPart::Text { text } if text == "complete"
    ));
}
