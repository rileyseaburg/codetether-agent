//! Tests for how the single-drain collector treats stream termination.
//!
//! Distinct from the restart engine (see `srp_tests`): this leaf collector has
//! no restart budget, so a premature end is rejected rather than committing a
//! response that may be truncated.

use super::collect_stream_completion_with_events;
use crate::provider::{ContentPart, StreamChunk};
use futures::{StreamExt, stream};

#[tokio::test]
async fn single_drain_rejects_partial_on_premature_end() {
    let s = Box::pin(stream::iter(vec![StreamChunk::Text("partial".to_string())]));
    let error = collect_stream_completion_with_events(s, None)
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "temporary provider availability issue; retry the request"
    );
    assert!(!error.to_string().to_ascii_lowercase().contains("websocket"));
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

#[tokio::test]
async fn done_finishes_without_waiting_for_transport_close() {
    let terminal = stream::iter(vec![
        StreamChunk::Text("complete".to_string()),
        StreamChunk::Done { usage: None },
    ]);
    let trailing = stream::once(async { panic!("collector polled past terminal Done") });
    let open = terminal.chain(trailing);
    let response = collect_stream_completion_with_events(Box::pin(open), None)
        .await
        .unwrap();
    assert!(matches!(
        &response.message.content[0],
        ContentPart::Text { text } if text == "complete"
    ));
}
