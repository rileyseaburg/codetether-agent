//! Original single-drain regression case.
//! Tests for how the single-drain collector treats stream termination.
//!
//! Distinct from the restart engine (see `srp_tests`): this leaf collector has
//! no restart budget, so a premature end is rejected rather than committing a
//! response that may be truncated.

use super::collect_stream_completion_with_events;

use codetether_provider_core::{ContentPart, StreamChunk};

use futures::{StreamExt, stream};

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
